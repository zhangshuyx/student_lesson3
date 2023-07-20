// 声明枚举类型
#[derive(Debug)]
enum Sex {
    Male,
    Female,
}

#[derive(Debug)]
enum SheTuan {
    Wenming,
    Kaifang,
    Chuangxin,
    Chuangzao,
}

#[derive(Debug)]
enum Class {
    Classone,
    Classtwo,
    Classthree,
}

#[derive(Debug)]
enum Course {
    English,
    Chinese,
    Math,
    Computer,
    History,
}

// 声明结构体
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    sex: Sex,
    shetuan: SheTuan,
    class: Class,
    course: Course,
}

// 声明结构体的方法
impl Student {
    fn print_info(&self) {
        println!("Info of {} is:\nAge: {}\nSex: {:?}\nShetuan: {:?}\nClass: {:?}\nCourse: {:?}\n", self.name, self.age, self.sex, self.shetuan, self.class, self.course)
    }
}

// 主函数，实例化结构体，调用方法，改变结构体内容
fn main() {
    let mut xiaoming: Student = Student{
        name : String::from("Xiaoming"),
        age : 20,
        sex : Sex::Male,
        shetuan : SheTuan::Chuangxin,
        class: Class::Classone,
        course: Course::Chinese,
    };

    let xiaohong = Student{
        name : String::from("Xiaohong"),
        age : 18,
        sex : Sex::Female,
        shetuan : SheTuan::Chuangzao,
        class: Class::Classtwo,
        course: Course::Computer,
    };

    xiaoming.print_info();
    xiaohong.print_info();

    // 修改结构体的项
    xiaoming.shetuan = SheTuan::Kaifang;
    xiaoming.age = 22;
    xiaoming.print_info();

    // 构建vector，消费vector
    let students = vec![xiaoming, xiaohong];
    for item in students {
        item.print_info()
    }

}