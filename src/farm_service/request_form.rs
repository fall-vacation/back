use rocket::FromForm;

const DEFAULT_PAGE_SIZE: i32 = 20;

#[derive(FromForm)]
pub struct ListForm {
    pub size: Option<i32>,
    pub offset: Option<i32>,
}

impl ListForm {
    pub fn get_farm_list_query(&self) -> String {
        let size = self.size.unwrap_or(DEFAULT_PAGE_SIZE);
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

    pub fn get_farm_review_list_query(&self, farm_id: i32) -> String {
        let size = self.size.unwrap_or(DEFAULT_PAGE_SIZE);
        let offset = self.offset.unwrap_or(0);
        format!("\
            SELECT \
                review_id, \
                farm_id, \
                user_id, \
                contents, \
                hit, \
                stars, \
                create_time, \
                modify_time, \
                delete_time \
            FROM farm_review \
            ORDER BY review_id \
            WHERE farm_id = {}
            LIMIT {} \
            OFFSET {}", farm_id, size, offset)
    }
}