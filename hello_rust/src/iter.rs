use core::num;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    for i in nums.iter() { // итератор
        println!("{}", i);
    }


    let nums2: Vec<_> = nums
        .iter()
        .map(|num| num * 2) // приминение к элементам
        .filter(|num| num % 2 == 3) // фильтр
        .collect();
    println!("{:?}", nums2);

    println!("{:?}", nums.iter().find(|num| num == &&3)); // поиск элемента, надо искать с типом && -> индекс
    println!("{:?}", nums.iter().count()); // считает элементы
    println!("{:?}", nums.iter().last()); // последний элемент
    println!("{:?}", nums.iter().sum::<i32>()); // сумма всех элементов
    println!("{:?}", nums.iter().min()); // минимальное значение
    println!("{:?}", nums.iter().max()); // максимальное значение

    let nums3: Vec<_> = nums
        .iter()
        .take(3) // берет первые n элементов
        .collect();

    let nums4: Vec<_> = nums
        .iter()
        .skip(3) // пропускает первые n элементов
        .collect();

    let numbers1 = vec![1, 2, 3];
    let numbers2 = vec![4, 5, 6];

    let x: Vec<_> = numbers1
        .iter()
        .zip(numbers2.iter()) // объединяет два итератора как кортеж (n1, n2)
        .collect();

}