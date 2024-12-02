use sea_orm::datetime;

struct characters{
    id: i32,
    name: String, 
    bio: String,
    age: i32,
    birthday: datetime,
    
}