use std::env;
// взято задание из лабы 1 и подогнано под тесты.


fn find_root(A: i32, B:i32, C:i32) -> String{
    let D = B * B - 4 * A * C;
    //let D = findD!(A, B, C);

    let mut result = String::new();
    
    if (D < 0){
        result = "Корней нет!".to_string();
    }

        
    else if (D == 0){
        let answer= -B / (2 * A);

        let answer = answer as f64;
        if (answer < 0.0){
            result = "Корней нет!".to_string();
        }
        else{
            //println!("x = {0}", answer.sqrt().round());
            result = "x = ".to_string() + &answer.sqrt().round().to_string();
        }
    }

    else if (D > 0){
        let D = D as f64;
        let B = B as f64;
        let A = A as f64;
        let answer1 = (-B - D.sqrt()) / (2.0 * A);
        let answer2 = (-B + D.sqrt()) / (2.0 * A);


   
        if (answer1 < 0.0 && answer2 < 0.0){ //квадрат не может быть отрицательным!
            result = "Корней нет!".to_string();
        }
        
        if (answer1 >= 0.0){
            result = "x1 = ".to_string() + &answer1.sqrt().round().to_string();
            //println!("x1 = {0}", answer1.sqrt().round());
        }
        if (answer2 >= 0.0){
            if(!result.is_empty()){result.push_str("; ");}// разделитель
            //println!("x2 = {0}", answer2.sqrt().round());
            let preres = "x2 = ".to_string() + &answer2.sqrt().round().to_string();
            result.push_str(&preres);
        }
    }

    result
}

#[cfg(test)]

mod tests{ // сами тесты.
    use std::result;

    use super::*;

    #[test]
    fn test_1(){
        let result = find_root(4,-5,1);
        assert_eq!(result, "x1 = 1; x2 = 1");
    }

    #[test]
    fn test_2(){
        let result = find_root(7,5,3);
        assert_eq!(result, "Корней нет!");
    }

    #[test]
    fn test_3(){
        let result = find_root(1,-2,1);
        assert_eq!(result, "x = 1");
    }
}


fn main(){

    let args: Vec<String> = env::args().collect();

    if(args.len() != 4){
        println!("Ошибка, неверное число параметров!");
        return
    }

    let A = args[1].parse::<i32>().unwrap(); // unwrap снимает try catch exeption - для упрощения
    let B = args[2].parse::<i32>().unwrap();
    let C = args[3].parse::<i32>().unwrap();

    println!("Введено уравнение {A}x^4 + {B}x^2 + {C} = 0");
    
    let res = find_root(A, B, C);
    println!("{res}");
}