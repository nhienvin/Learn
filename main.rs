fn main(){
    //Mỗi biến được khai báo mà không khởi tạo thì khi sử dụng sẽ lỗi
    //Khai báo nhưng không dùng sẽ có cảnh báo, để loại bỏ cảnh báo thì thêm _ trước biến đó. Ví dụ _y
    let x: i32;
    let y: i32;
    assert_eq!(x, 5);
    println!("Thành công!");
}