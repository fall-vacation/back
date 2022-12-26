use sqlx::postgres::PgRow;

pub trait DaoStruct{
    type Dto: DtoStruct;

    fn match_pg_row(row: &PgRow) -> Self;
    fn to_dto(self) -> Self::Dto;

    fn to_vec_dao(rows : Vec<PgRow>) -> Vec<Self>
        where Self: Sized
    {
        let mut output = Vec::new();
        for each in rows {
            output.push(Self::match_pg_row(&each));
        }
        output
    }

    fn to_vec_dto(rows : Vec<PgRow>) -> Vec<Self::Dto>
        where Self: Sized
    {
        let mut output = Vec::new();
        for each in rows {
            output.push(Self::match_pg_row(&each).to_dto());
        }
        output
    }
}

pub trait DtoStruct {
    type Dao : DaoStruct;

    fn to_dao(self) -> Self::Dao;

    fn to_vec_dto(rows : Vec<Self::Dao>) -> Vec<<<Self as DtoStruct>::Dao as DaoStruct>::Dto>
    {
        let mut output = Vec::new();
        for each in rows {
            output.push(each.to_dto());
        }
        output
    }
}