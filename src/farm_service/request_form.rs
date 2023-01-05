use rocket::FromForm;

#[derive(FromForm)]
pub struct ListForm {
    pub size: Option<i32>,
    pub offset: Option<i32>,
}

impl ListForm {
    pub fn get_list_query(&self) -> String {
        let size = self.size.unwrap_or(20);
        let offset = self.offset.unwrap_or(0);
        format!("\
            SELECT \
                farm_id, \
                farm_name, \
                farm_address, \
                farm_address_div, \
                farm_owner_name, \
                farm_owner_phone, \
                price, \
                stars, \
                available_use_start, \
                available_use_end, \
                available_lesson, \
                etc \
            FROM farm \
            ORDER BY farm_id \
            LIMIT {} \
            OFFSET {}", size, offset)
    }
}