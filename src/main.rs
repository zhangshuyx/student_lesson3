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

#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    sex: Sex,
    shetuan: SheTuan,
    class: Class,
    course: Course,
}

impl Student {
    fn print_info(&self) {
        println!("Info of {} is:\nAge: {}\nSex: {:?}\nShetuan: {:?}\nClass: {:?}\nCourse: {:?}\n", self.name, self.age, self.sex, self.shetuan, self.class, self.course)
    }
}

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

    xiaoming.shetuan = SheTuan::Kaifang;
    xiaoming.age = 22;
    xiaoming.print_info();

    
}