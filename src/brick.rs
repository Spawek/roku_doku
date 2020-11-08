use lazy_static::lazy_static;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

pub fn xy(x: i32, y: i32) -> XY {
    XY { x, y }
}

// Offsets are assumed to be >= 0.
// Min offset must be = 0 for both x and y.
#[derive(Clone, Debug)]
pub struct Brick {
    pub offsets: Vec<XY>,
}

fn normalize_brick(brick: &Brick) -> Brick {
    let min_x = brick.offsets.iter().min_by_key(|v| v.x).unwrap().x;
    let min_y = brick.offsets.iter().min_by_key(|v| v.y).unwrap().y;

    Brick {
        offsets: brick
            .offsets
            .iter()
            .map(|v| xy(v.x - min_x, v.y - min_y))
            .collect(),
    }
}

fn rotate_clockwise(brick: &Brick) -> Brick {
    let max_x = brick.offsets.iter().max_by_key(|v| v.x).unwrap().x;
    let max_y = brick.offsets.iter().max_by_key(|v| v.y).unwrap().y;
    let max = std::cmp::max(max_x, max_y);

    if max == 0 {
        return brick.clone();
    }

    if max == 1 {
        return normalize_brick(&Brick {
            offsets: brick
                .offsets
                .iter()
                .map(|v| {
                    if v.x == 0 && v.y == 0 {
                        xy(1, 0)
                    } else if v.x == 1 && v.y == 0 {
                        xy(1, 1)
                    } else if v.x == 1 && v.y == 1 {
                        xy(0, 1)
                    } else if v.x == 0 && v.y == 1 {
                        xy(0, 0)
                    } else {
                        panic!()
                    }
                })
                .collect(),
        });
    }

    if max == 2 {
        return normalize_brick(&Brick {
            offsets: brick
                .offsets
                .iter()
                .map(|v| {
                    if v.x == 0 && v.y == 0 {
                        xy(2, 0)
                    } else if v.x == 2 && v.y == 0 {
                        xy(2, 2)
                    } else if v.x == 2 && v.y == 2 {
                        xy(0, 2)
                    } else if v.x == 0 && v.y == 2 {
                        xy(0, 0)
                    } else if v.x == 1 && v.y == 0 {
                        xy(2, 1)
                    } else if v.x == 2 && v.y == 1 {
                        xy(1, 2)
                    } else if v.x == 1 && v.y == 2 {
                        xy(0, 1)
                    } else if v.x == 0 && v.y == 1 {
                        xy(1, 0)
                    } else if v.x == 1 && v.y == 1 {
                        xy(1, 1)
                    } else {
                        panic!()
                    }
                })
                .collect(),
        });
    }

    // long lines are handled in a special way, as there are only 2 bricks not fitting in 3x3 box
    if (max_x == 4 && max_y == 0) || (max_y == 4 && max_x == 0) ||
        (max_x == 3 && max_y == 0) || (max_y == 3 && max_x == 0)
    {
        return Brick {
            offsets: brick.offsets.iter().map(|v| xy(v.y, v.x)).collect(),
        };
    }

    panic!("unsupported brick: {:#?}", brick);
}

fn all_brick_rotations(brick: &Brick) -> Vec<Brick> {
    let b1 = brick.clone();
    let b2 = rotate_clockwise(&b1);
    let b3 = rotate_clockwise(&b2);
    let b4 = rotate_clockwise(&b3);

    vec![b1, b2, b3, b4]
}

fn generate_brick_library() -> Vec<Brick> {
    // X
    let brick_0 = Brick {
        offsets: vec![xy(0, 0)],
    };

    // XX
    let brick_1 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0)],
    };

    // XXX
    let brick_2 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0), xy(2, 0)],
    };

    // XXXX
    let brick_3 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0), xy(2, 0), xy(3, 0)],
    };

    // XXXXX
    let brick_4 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0), xy(2, 0), xy(3, 0), xy(4, 0)],
    };

    // XX
    // X
    let brick_5 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0), xy(0, 1)],
    };

    // XXX
    //  X
    let brick_6 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0), xy(2, 0), xy(1, 1)],
    };

    // XXX
    //  X
    //  X
    let brick_7 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0), xy(2, 0), xy(1, 1), xy(1, 2)],
    };

    // XXX
    // X X
    let brick_8 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0), xy(2, 0), xy(0, 1), xy(2, 1)],
    };

    // XXX
    // X X
    // X X
    let brick_9 = Brick {
        offsets: vec![
            xy(0, 0),
            xy(1, 0),
            xy(2, 0),
            xy(0, 1),
            xy(2, 1),
            xy(0, 2),
            xy(2, 2),
        ],
    };

    // XX
    //  XX
    let brick_10 = Brick {
        offsets: vec![xy(0, 0), xy(1, 0), xy(1, 1), xy(2, 1)],
    };

    let mut ret = vec![];
    ret.extend(all_brick_rotations(&brick_0));
    ret.extend(all_brick_rotations(&brick_1));
    ret.extend(all_brick_rotations(&brick_2));
    ret.extend(all_brick_rotations(&brick_3));
    ret.extend(all_brick_rotations(&brick_4));
    ret.extend(all_brick_rotations(&brick_5));
    ret.extend(all_brick_rotations(&brick_6));
    ret.extend(all_brick_rotations(&brick_7));
    ret.extend(all_brick_rotations(&brick_8));
    ret.extend(all_brick_rotations(&brick_9));
    ret.extend(all_brick_rotations(&brick_10));

    ret
}

pub fn random_brick() -> Brick {
    lazy_static! {
        static ref LIBRARY: Vec<Brick> = generate_brick_library();
    }
    let mut rng = rand::thread_rng();
    LIBRARY[rng.gen_range(0, LIBRARY.len())].clone()
}

pub fn print_brick(brick: &Brick) {
    let max_x = brick.offsets.iter().max_by_key(|v|v.x).unwrap().x;
    let max_y = brick.offsets.iter().max_by_key(|v|v.y).unwrap().y;

    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            let c = if brick.offsets.iter().find(|v| v.x == x && v.y == y).is_some(){
                'X'
            } else {' '};

            print!("{}", c);
        }
        println!();
    }
}

pub fn print_all_bricks() {
    let bricks = generate_brick_library();

    println!("printing {} bricks from the library", bricks.len());
    let mut i = 0;
    for brick in bricks {
        println!("brick {}:", &i);
        i = i+1;

        print_brick(&brick);
        println!("----------------");
    }
}
