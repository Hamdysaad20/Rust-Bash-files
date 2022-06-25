# Scalar data type (integers, floats, boolean, char) immutable

## integers :
- u (no negative) 
- i (positive & negative)
//types : 8,16,32,64,128
### EX: 

```
let num:u32=16;
let num:i16=-136;
```

### Note: 8:0-255, 16:0-65,535, 32:-2m-2m .
___________
## floats :
- f32 
- f64(default) (only)
 ### EX:
 ```rust
let num:f32=16.23;
```
___________
## boolean :
- bool
### EX:
```rust
let is_num:bool=true;
```
### Note: you can use 0|1 instead  .
__________
## char :
- char
### EX:
```rust
let letter:char='a';
```
### Note: any type as long as it's single_quotation||one_char  .
______________________________________________________________________________________________
# Compound data type (tuple , array) immutable


## tuple 
### EX:
```rust
let tuple_text:(i32,bool,bool,f32,char)=(1,true,false,4.23,'S'); 

```

### Note: to assign a tuple to another ,they should be the same type tho.
Ex:
```rust
let tuple_text:(i32)=(1) != let tuple_text:(i32)=(1) 
``` 
### they aren't the same
________
### acess tuple :
- YOU CAN'T ACESS THEM ALL !!!!
- so , instead use the index 
### EX:
```rust
let tup:(i32,char)=(3,'d');
println("{}",tup.0);
//output: 3 
```
__________
## array:
### Note: you have to specify the [type;size] & they have to be the same type .
### EX:
```rust
let arr:[i32;4]=[1,2,3,4];
```
### Note: access elements by there Indices. && can't add more elements to the array, i have to create a new array 
_________________________________
- if i need to change any of these values i should make them mutable by putting( mut ) firest then assign the new value

### EX:
```rust
let mut text:String ="Hi, MOM!";
text:String="Hi, DAD!";
```
LOL that's it .
thank you 
