use std::fs;

fn main() {
    let images_bytes = fs::read("./train-images-idx3-ubyte").unwrap();
    let labels_bytes = fs::read("./train-labels-idx1-ubyte").unwrap();

    let mut image_bytes = images_bytes.iter();
    let mut label_types = labels_bytes.iter();

    for _ in 0..((4 * 2) + 2) {
        label_types.next().unwrap();
    }

    for _ in 0..(4 * 4 + 2) {
        image_bytes.next().unwrap();
    }

    for _ in 0..12 {
        println!("label: {}", label_types.next().unwrap());

        for _ in 0..28 {
            for _ in 0..28 {
                let byte = image_bytes.next().unwrap().to_string();

                for _ in 0..(byte.len()) {
                    print!(" ");
                }

                for _ in 0..(3 - byte.len()) {
                    print!("▒");
                }

                // print!(" ");

                // print!("{} ", byte)
            }

            println!();
        }

        println!();
    }

    //
    //

    // for file in file_bytes {
    //     println!("{}", file);
    // }

    // println!("{}", file);
}
