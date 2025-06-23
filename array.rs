/*
*   ARRAYS
*   is a homogeneous collection of values, - collection of same datatypes
   -------------------------------------------------------------------------------
    -An array declaration allocates sequential memory blocks.
    -Arrays are static. This means that an array once initialized cannot be resized.
    -Each memory block represents an array element.
    -Array elements are identified by a unique integer called the subscript/ index of the element.
    -Populating the array elements is known as array initialization.
    -useArray element values can be updated or modified but cannot be deleted.
------------------------------------------------------------------------------------------------------
-    SYNTAX1
    let var_name= [value1,value2,value3];

    SYNTAX2
    let var_name : [datatype,size ] =[value1,value2,value3];
    ---size---size of how much can it store
    SYNTAX3
    let var_name:[dataType;size] = [default_value_for_elements,size];


*
*
*
* */

fn main() {
    let arr: [u32; 4] = [10, 20, 30, 40];
    println!("{:?}", arr);
    println!("the lenght of array is {}", arr.len());
    let arr1 = [10, 20, 30, 40];
    println!("Array without assigned dataT and lenght ,{:?}", arr1);
    let arr2: [i32; 4] = [-1; 4];
    println!("This is a default value array: {:?}", arr2);
}
