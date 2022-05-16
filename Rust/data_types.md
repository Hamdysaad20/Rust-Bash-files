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
 ```
let num:f32=16.23;
```
___________
## boolean :
- bool
### EX:
```
let is_num:bool=true;
```
### Note: you can use 0|1 instead  .
__________
## char :
- char
### EX:
```
let letter:char='a';
```
### Note: any type as long as it's single_quotation||one_char  .
______________________________________________________________________________________________
# Compound data type (tuple , array) immutable


## tuple 
### EX:
```
let tuple_text:(i32,bool,bool,f32,char)=(1,true,false,4.23,'S'); 

```

### Note: to assign a tuple to another ,they should be the same type tho.
Ex:
```
let tuple_text:(i32)=(1) != let tuple_text:(i32)=(1) 
``` 
### they aren't the same
________
### acess tuple :
- YOU CAN'T ACESS THEM ALL !!!!
- so , instade use the index 
### EX:
```
let tup:(i32,char)=(3,'d');
println("{}",tup.0);
//output: 3 
```
