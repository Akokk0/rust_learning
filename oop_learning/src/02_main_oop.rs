use oop_learning::Draw;
use oop_learning::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制一个选择框
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK")
            }),
            // Box::new(String::from("hello"))
        ]
    };

    screen.run();
}