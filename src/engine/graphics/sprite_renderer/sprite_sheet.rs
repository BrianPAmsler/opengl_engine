use std::{collections::{BTreeSet, HashMap, HashSet, VecDeque}, fmt::Debug, hash::Hash, ops::{Deref, DerefMut}};

use lazy_static::lazy_static;

use crate::engine::graphics::{image::Image, Graphics, Texture};

struct SpriteCell {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    name: String
}

impl Hash for SpriteCell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for SpriteCell {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for SpriteCell {}

struct ImageCell {
    img: Image, 
    name: String
}

impl ImageCell {
    fn size(&self) -> u32 {
        self.img.width().max(self.img.height())
    }
}

impl Ord for ImageCell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size().cmp(&other.size()).reverse()
    }
}

impl PartialOrd for ImageCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size().partial_cmp(&other.size()).map(|c| c.reverse())
    }
}

impl PartialEq for ImageCell {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
    }
}

impl Eq for ImageCell {}

pub struct SpriteSheetBuilder {
    max_texture_size: u32,
    sprites: BTreeSet<ImageCell>
}

impl Debug for SpriteSheetBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpriteSheetBuilder").field("max_texture_size", &self.max_texture_size).finish()
    }
}

impl SpriteSheetBuilder {
    pub fn new(max_texture_size: u32) -> SpriteSheetBuilder {
        SpriteSheetBuilder { max_texture_size, sprites: BTreeSet::new() }
    }

    pub fn split(self) -> (SpriteSheetBuilder, SpriteSheetBuilder) {
        let mut set1 = BTreeSet::new();
        let mut set2 = BTreeSet::new();

        self.sprites.into_iter().enumerate().for_each(|(i, sprite)| if i % 2 == 0 {
            set1.insert(sprite);
        } else {
            set2.insert(sprite);
        });

        let max_texture_size = self.max_texture_size;

        (SpriteSheetBuilder { max_texture_size, sprites: set1}, SpriteSheetBuilder { max_texture_size, sprites: set2 })
    }

    pub fn add_image(&mut self, img: Image, name: String) {
        self.sprites.insert(ImageCell { img, name });
    }

    pub fn try_build(self) -> Result<SpriteSheet, SpriteSheetBuilder> {
        struct Node<'a> {
            x: u32,
            y: u32,
            w: u32,
            h: u32,
            img: Option<&'a ImageCell>,
            right: Option<Box<Node<'a>>>,
            down: Option<Box<Node<'a>>>
        }

        impl Debug for Node<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Node").field("x", &self.x).field("y", &self.y).field("w", &self.w).field("h", &self.h).field("img", &self.img.map(|img| (img.img.width(), img.img.height()))).field("right", &self.right).field("down", &self.down).finish()
            }
        }

        let first_image = match self.sprites.iter().nth(0) {
            Some(s) => s,
            None => return Err(self),
        };

        let initial_size = first_image.img.width().max(first_image.img.height());
        let mut root = Node { x: 0, y: 0, w: initial_size, h: initial_size, img: None, right: None, down:  None };

        lazy_static! { static ref EMPTY_CELL: ImageCell = ImageCell { img: Image::empty(0, 0), name: String::from("") }; }

        fn find_node<'a, 'b>(root: &'a mut Node<'b>, width: u32, height: u32) -> Option<&'a mut Node<'b>> {
            match root {
                Node { img: Some(_), .. } => {
                    let right = match &mut root.right {
                        Some(node) => find_node(node.deref_mut(), width, height),
                        None => None,
                    };

                    match right {
                        Some(s) => Some(s),
                        None => match &mut root.down {
                            Some(node) => find_node(node.deref_mut(), width, height),
                            None => None
                        }
                    }
                },
                Node { img: None, .. } => {
                    if width <= root.w && height <= root.h {
                        Some(root)
                    } else {
                        None
                    }
                }
            }
        }

        fn insert_img<'a, 'b>(node: &'a mut Node<'b>, img: &'b ImageCell) {
            let w = img.img.width();
            let h = img.img.height();

            match node {    
                Node { img: None, .. } => {
                    node.img = Some(img);
                    node.down = Some(Box::new(
                        Node {
                            x: node.x,
                            y: node.y + h,
                            w: node.w,
                            h: node.h - h,
                            img: None,
                            right: None,
                            down: None,
                        }
                    ));
                    node.right = Some(Box::new(
                        Node {
                            x: node.x + w,
                            y: node.y,
                            w: node.w - w,
                            h,
                            img: None,
                            right: None,
                            down: None,
                        }
                    ))
                },
                _ => panic!("node already used!")
            }
        }

        fn grow_right<'a, 'b>(root: &'a mut Node<'b>, img: &'b ImageCell) -> &'a mut Node<'b> {
            println!("grow right");
            let w = img.img.width();
            let h = img.img.height();

            let old_root = Node {
                x: root.x,
                y: root.y,
                w: root.w,  
                h: root.h,
                img: root.img.take(),
                right: root.right.take(),
                down: root.down.take(),
            };

            *root = Node {
                x: 0,
                y: 0,
                w: old_root.w + w,
                h: old_root.h,
                img: Some(&EMPTY_CELL),
                right: Some(Box::new(
                    Node {
                        x: old_root.w,
                        y: 0,
                        w,
                        h: old_root.h,
                        img: None,
                        right: None,
                        down: None,
                    }
                )),
                down: Some(Box::new(old_root)),
            };

            match find_node(root, w, h) {
                Some(node) => {
                    insert_img(node, img);
                    node
                },
                None => panic!("this shouldn't happen!"),
            }
        }

        fn grow_down<'a, 'b>(root: &'a mut Node<'b>, img: &'b ImageCell) -> &'a mut Node<'b> {
            println!("grow down");
            let w = img.img.width();
            let h = img.img.height();

            let old_root = Node {
                x: root.x,
                y: root.y,
                w: root.w,  
                h: root.h,
                img: root.img.take(),
                right: root.right.take(),
                down: root.down.take(),
            };

            *root = Node {
                x: 0,
                y: 0,
                w: old_root.w,
                h: old_root.h + h,
                img: Some(&EMPTY_CELL),
                down: Some(Box::new(
                    Node {
                        x: 0,
                        y: old_root.h,
                        w: old_root.w,
                        h,
                        img: None,
                        right: None,
                        down: None,
                    }
                )),
                right: Some(Box::new(old_root)),
            };

            match find_node(root, w, h) {
                Some(node) => {
                    insert_img(node, img);
                    node
                },
                None => panic!("this shouldn't happen!"),
            }
        }

        for img in &self.sprites {
            let width = img.img.width();
            let height = img.img.height();
            match find_node(&mut root, width, height) {
                Some(node) => {
                    println!("don't grow");
                    insert_img(node, img);
                },
                None => {
                    let can_grow_down = width <= root.w;
                    let can_grow_right = height <= root.h;

                    let should_grow_right = can_grow_right && (root.h >= root.w + width);
                    let should_grow_down = can_grow_down && (root.w >= root.h + height);

                    if should_grow_right {
                        grow_right(&mut root, img);
                    } else if should_grow_down {
                        grow_down(&mut root, img);
                    } else if can_grow_right {
                        grow_right(&mut root, img);
                    } else if can_grow_down {
                        grow_down(&mut root, img);
                    } else {
                        panic!("This should not happen!")
                    };
                },
            }
        }

        if root.w > self.max_texture_size || root.h > self.max_texture_size {
            drop(root);
            return Err(self);
        }

        let mut final_sheet = Image::empty(root.w + 1, root.h + 1);

        let mut sprite_list = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(root);
        while !q.is_empty() {
            let mut node = q.pop_front().unwrap();

            match node.img.take() {
                Some(ImageCell { img, name }) => {
                    if img.width() > 0 && img.height() > 0 {
                        let sprite = SpriteCell {
                            x: node.x,
                            y: node.y,
                            width: img.width(),
                            height: img.height(),
                            name: name.clone(),
                        };

                        final_sheet.blit(&img, sprite.x, sprite.y);
                        sprite_list.push(sprite);
                    }

                    match node.right.take() {
                        Some(node) => q.push_back(*node),
                        None => (),
                    }

                    match node.down.take() {
                        Some(node) => q.push_back(*node),
                        None => (),
                    }
                },
                    None => (),
                }
        }
        
        Ok(SpriteSheet { sheet: final_sheet, sprites: sprite_list })
    }

    pub fn build(self) -> Vec<SpriteSheet> {
        let mut sheets = Vec::new();

        match self.try_build() {
            Ok(sheet) => sheets.push(sheet),
            Err(builder) => {
                let (a, b) = builder.split();

                sheets.extend(a.build());
                sheets.extend(b.build());
            }
        }

        sheets
    }
}

pub struct SpriteSheet {
    sheet: Image,
    sprites: Vec<SpriteCell>
}

impl SpriteSheet {
    pub fn as_texture(self, gfx: &Graphics) -> Texture {
        Texture::new(gfx, self.sheet.data(), self.sheet.width(), self.sheet.height())
    }

    pub fn image(&self) -> &Image {
        &self.sheet
    }

    pub fn create_index(&self) -> HashMap<String, u32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, fs::OpenOptions};

    use pathbuf::pathbuf;

    use crate::engine::graphics::image::Image;

    use super::SpriteSheetBuilder;

    #[test]
    #[ignore="output must be manually verified"]
    fn spritesheet_build() {
        let dir = std::fs::read_dir(pathbuf!("test_files", "misc", "test_sprites")).unwrap();
        let mut files = Vec::new();
        for file in dir {
            match file {
                Ok(file) => match file.path().extension() {
                    Some(ext)=> if ext == OsStr::new("png") {
                        files.push(file.path());
                    },
                    None => (),
                },
                Err(_) => (),
            }
        }

        let mut builder = SpriteSheetBuilder::new(100000000);
        files.into_iter().map(|file| (Image::load_from_file(&file).unwrap(), file.file_name().unwrap().to_str().unwrap().to_owned()))
        .for_each(|(img, name)| {
            builder.add_image(img, name);
        });

        let sprite_sheet = builder.try_build().unwrap();
        
        let mut sheet_file = OpenOptions::new().create(true).write(true).open(pathbuf!("test_files", "output", "sprite_sheet.png")).unwrap();
        
        let out = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(sprite_sheet.image().width(), sprite_sheet.image().height(), sprite_sheet.image().data()).unwrap();
        out.write_to(&mut sheet_file, image::ImageFormat::Png).unwrap();

        panic!("This test is not automated. Manually verify the result at: test_files/output/sprite_sheet.png");
    }
}