use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 创建一个 Rc 指针
    let data = Rc::new(5);
    println!("Count after creation: {}!",Rc::strong_count(&data));
    // 复制 Rc 不会复制数据，只会增加引用
    let _data_clone = data.clone();
    println!("Count after clone: {}!",Rc::strong_count(&data));
    // 当克隆被释放时，引用会减少
    {
        let _data_clone_clone = data.clone();
        println!("Count after twice clone: {}!",Rc::strong_count(&data));
    }
    println!("Count after twice clone and free one: {}!",Rc::strong_count(&data));


    let cell = RefCell::new(12);
    // 不可变借用
    {
        let cell_value = cell.borrow();
        println!("cell_value is: {}", *cell_value);
    }
    // 可变借用
    {
        let mut cell_value_mutable = cell.borrow_mut();
        *cell_value_mutable += 1;
    }
    println!("cell_value after change is: {}", *cell.borrow());

}
