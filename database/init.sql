CREATE TABLE FARM(
                     farm_id serial NOT NULL PRIMARY KEY,
                     farm_name varchar(50) NOT NULL,
                     farm_address varchar(50),
                     farm_address_div integer,
                     farm_owner_name varchar(50),
                     farm_owner_phone varchar(50),
                     price varchar(50),
                     stars double precision,
                     available_use_start time,
                     available_use_end time,
                     available_lesson boolean,
                     etc text
);
COMMENT ON TABLE FARM IS '농장 정보';
COMMENT ON COLUMN FARM.farm_id IS 'ID';
COMMENT ON COLUMN FARM.farm_name IS '농장명';
COMMENT ON COLUMN FARM.farm_address IS '농장 주소';
COMMENT ON COLUMN FARM.farm_owner_name IS '농장주 이름';
COMMENT ON COLUMN FARM.farm_owner_phone IS '농장주 번호';
COMMENT ON COLUMN FARM.price IS '가격(String)';
COMMENT ON COLUMN FARM.stars IS '평균 별점';
COMMENT ON COLUMN FARM.available_use_start IS '사용 가능 시작 시간';
COMMENT ON COLUMN FARM.available_use_end IS '사용 가능 종료 시간';
COMMENT ON COLUMN FARM.available_lesson IS '강습 가능 여부';
COMMENT ON COLUMN FARM.etc IS '그 외 참고사항 (농장주 설명 등)';
-- 주소 검색 시에는 Regex 설정해서 검색하는 것으로 추가해야 할 듯
-- 필터링 기능에는 아마 도/시/구/군/읍/면/동 등의 구분이 필요함 (가능한지 모르겠다)

CREATE TABLE FARM_URLS(
                          farm_urls_id serial NOT NULL PRIMARY KEY,
                          farm_id integer NOT NULL,
                          url_division varchar(50) NOT NULL,
                          url varchar(2048) NOT NULL
);
COMMENT ON TABLE FARM_URLS IS '농장 URL 정보';
COMMENT ON COLUMN FARM_URLS.farm_urls_id IS 'ID';
COMMENT ON COLUMN FARM_URLS.farm_id IS '농장 ID';
COMMENT ON COLUMN FARM_URLS.url_division IS 'URL 구분';
COMMENT ON COLUMN FARM_URLS.url IS 'URL';
-- 네이버, 카카오 주소 url + sns, 홈피 url 등

CREATE TABLE ADDRESS_DIVISION(
                                 addr_div_id serial NOT NULL PRIMARY KEY,
                                 div_1 varchar(50) NOT NULL,
                                 div_2 varchar(50)
);
COMMENT ON TABLE ADDRESS_DIVISION IS '농장 URL 정보';
COMMENT ON COLUMN ADDRESS_DIVISION.addr_div_id IS 'ID';
COMMENT ON COLUMN ADDRESS_DIVISION.div_1 IS '주소 1 deps';
COMMENT ON COLUMN ADDRESS_DIVISION.div_2 IS '주소 2 deps';

CREATE TABLE FARM_USING_CROPS(
                                 farm_id integer NOT NULL,
                                 crop_id integer NOT NULL,
                                 PRIMARY KEY (farm_id, crop_id)
);
COMMENT ON TABLE FARM_USING_CROPS IS '농장-작물 간 n:n 관계';
COMMENT ON COLUMN FARM_USING_CROPS.farm_id IS '농장 ID';
COMMENT ON COLUMN FARM_USING_CROPS.crop_id IS '작물 ID';

CREATE TABLE CROP(
                     crop_id serial NOT NULL PRIMARY KEY,
                     crop_name varchar(50) NOT NULL,
                     crop_image_url varchar(2048),
                     difficulty smallint,
                     growth_start smallint,
                     growth_end smallint,
                     temperature real,
                     crop_desc text,
                     crop_caution text
);
COMMENT ON TABLE CROP IS '작물 정보';
COMMENT ON COLUMN CROP.crop_id IS 'ID';
COMMENT ON COLUMN CROP.crop_name IS '작물명';
COMMENT ON COLUMN CROP.crop_image_url IS '작물 이미지';
COMMENT ON COLUMN CROP.difficulty IS '난이도';
COMMENT ON COLUMN CROP.growth_start IS '재배 시작 시기';
COMMENT ON COLUMN CROP.growth_end IS '재배 종료 시기';
COMMENT ON COLUMN CROP.temperature IS '온도';
COMMENT ON COLUMN CROP.crop_desc IS '설명';
COMMENT ON COLUMN CROP.crop_caution IS '주의사항';

CREATE TABLE FV_USER(
                        user_id serial NOT NULL PRIMARY KEY,
                        email_address varchar(50) NOT NULL,
                        user_nickname varchar(50) NOT NULL,
                        user_role varchar(50) NOT NULL,
                        user_image varchar(50),
                        user_farm_id integer,
                        access_token varchar(50),
                        access_expired timestamp,
                        refresh_token varchar(50),
                        refresh_expired timestamp
);
COMMENT ON TABLE FV_USER IS '사용자 정보';
COMMENT ON COLUMN FV_USER.user_id IS 'ID';
COMMENT ON COLUMN FV_USER.email_address IS 'e-mail';
COMMENT ON COLUMN FV_USER.user_nickname IS '닉네임';
COMMENT ON COLUMN FV_USER.user_role IS '사용자 구분';
COMMENT ON COLUMN FV_USER.user_image IS '사용자 이미지';
COMMENT ON COLUMN FV_USER.user_farm_id IS '농장주인 경우 본인의 농장 ID';
COMMENT ON COLUMN FV_USER.access_token IS '엑세스 토큰';
COMMENT ON COLUMN FV_USER.access_expired IS '엑세스 토큰 만료 시기';
COMMENT ON COLUMN FV_USER.refresh_token IS '리프레시 토큰';
COMMENT ON COLUMN FV_USER.refresh_expired IS '리프레시 토큰 만료 시기';

CREATE TABLE FV_USER_INFO(
                             user_id integer NOT NULL PRIMARY KEY,
                             my_farm_review_ids jsonb,
                             my_crop_tips_ids jsonb,
                             my_crop_tips_rec_ids jsonb,
                             my_crop_tips_not_rec_ids jsonb,
                             my_free_board_ids jsonb,
                             my_free_board_rec_ids jsonb,
                             my_free_board_not_rec_ids jsonb,
                             my_free_board_comment_ids jsonb
);
COMMENT ON TABLE FV_USER_INFO IS '사용자 활동 정보';
COMMENT ON COLUMN FV_USER_INFO.user_id IS '사용자 ID';
COMMENT ON COLUMN FV_USER_INFO.my_farm_review_ids IS '내가 쓴 농장 리뷰';
COMMENT ON COLUMN FV_USER_INFO.my_crop_tips_ids IS '내가 쓴 작물 팁';
COMMENT ON COLUMN FV_USER_INFO.my_crop_tips_rec_ids IS '내가 누른 작물 팁 추천';
COMMENT ON COLUMN FV_USER_INFO.my_crop_tips_not_rec_ids IS '내가 누른 작물 팁 비추천';
COMMENT ON COLUMN FV_USER_INFO.my_free_board_ids IS '내가 쓴 자유게시판 글';
COMMENT ON COLUMN FV_USER_INFO.my_free_board_rec_ids IS '내가 누른 자유게시판 추천';
COMMENT ON COLUMN FV_USER_INFO.my_free_board_not_rec_ids IS '내가 누른 자유게시판 비추천';
COMMENT ON COLUMN FV_USER_INFO.my_free_board_comment_ids IS '내가 쓴 게시판 댓글';

CREATE TABLE FARM_REVIEW(
                            review_id serial NOT NULL PRIMARY KEY,
                            farm_id integer NOT NULL,
                            user_id integer NOT NULL,
                            contents text,
                            hit integer,
                            stars integer,
                            create_time timestamp,
                            modify_time timestamp,
                            delete_time timestamp
);
COMMENT ON TABLE FARM_REVIEW IS '농장 리뷰';
COMMENT ON COLUMN FARM_REVIEW.review_id IS 'ID';
COMMENT ON COLUMN FARM_REVIEW.farm_id IS '농장 ID';
COMMENT ON COLUMN FARM_REVIEW.user_id IS '사용자 ID';
COMMENT ON COLUMN FARM_REVIEW.contents IS '내용';
COMMENT ON COLUMN FARM_REVIEW.hit IS '조회수';
COMMENT ON COLUMN FARM_REVIEW.stars IS '별점';
COMMENT ON COLUMN FARM_REVIEW.create_time IS '생성 일자';
COMMENT ON COLUMN FARM_REVIEW.modify_time IS '수정 일자';
COMMENT ON COLUMN FARM_REVIEW.delete_time IS '삭제 일자';

CREATE TABLE CROP_TIP(
                         tip_id serial NOT NULL PRIMARY KEY,
                         crop_id integer NOT NULL,
                         user_id integer NOT NULL,
                         contents text,
                         hit integer,
                         rec_count integer,
                         not_rec_count integer,
                         create_time timestamp,
                         modify_time timestamp,
                         delete_time timestamp
);
COMMENT ON TABLE CROP_TIP IS '작물 팁';
COMMENT ON COLUMN CROP_TIP.tip_id IS 'ID';
COMMENT ON COLUMN CROP_TIP.crop_id IS '작물 ID';
COMMENT ON COLUMN CROP_TIP.user_id IS '사용자 ID';
COMMENT ON COLUMN CROP_TIP.contents IS '내용';
COMMENT ON COLUMN CROP_TIP.hit IS '조회수';
COMMENT ON COLUMN CROP_TIP.rec_count IS '추천수';
COMMENT ON COLUMN CROP_TIP.not_rec_count IS '비추천수';
COMMENT ON COLUMN CROP_TIP.create_time IS '생성 일자';
COMMENT ON COLUMN CROP_TIP.modify_time IS '수정 일자';
COMMENT ON COLUMN CROP_TIP.delete_time IS '삭제 일자';

CREATE TABLE TAGS(
                     tag_id serial NOT NULL PRIMARY KEY,
                     tag varchar(50) NOT NULL
);
COMMENT ON TABLE TAGS IS '태그 정보';
COMMENT ON COLUMN TAGS.tag_id IS 'ID';
COMMENT ON COLUMN TAGS.tag IS '태그명';

CREATE TABLE USING_TAGS_INFO(
                                using_tags_info_id serial NOT NULL PRIMARY KEY,
                                table_id varchar(50) NOT NULL,
                                in_using_id integer NOT NULL,
                                tag_ids jsonb
);
COMMENT ON TABLE USING_TAGS_INFO IS '태그 사용 정보';
COMMENT ON COLUMN USING_TAGS_INFO.using_tags_info_id IS 'ID';
COMMENT ON COLUMN USING_TAGS_INFO.table_id IS '테이블명';
COMMENT ON COLUMN USING_TAGS_INFO.in_using_id IS '테이블 내 사용 ID';
COMMENT ON COLUMN USING_TAGS_INFO.tag_ids IS '사용중인 태그들';

CREATE TABLE FREE_BOARD(
                           free_board_id serial NOT NULL PRIMARY KEY,
                           title text NOT NULL,
                           user_id integer NOT NULL,
                           contents text,
                           hits integer,
                           rec_count integer,
                           not_rec_count integer,
                           create_time timestamp,
                           modify_time timestamp,
                           delete_time timestamp
);
COMMENT ON TABLE FREE_BOARD IS '자유게시판';
COMMENT ON COLUMN FREE_BOARD.free_board_id IS 'ID';
COMMENT ON COLUMN FREE_BOARD.title IS '제목';
COMMENT ON COLUMN FREE_BOARD.user_id IS '작성자 ID';
COMMENT ON COLUMN FREE_BOARD.contents IS '내용';
COMMENT ON COLUMN FREE_BOARD.hits IS '조회수';
COMMENT ON COLUMN FREE_BOARD.rec_count IS '추천수';
COMMENT ON COLUMN FREE_BOARD.not_rec_count IS '비추천수';
COMMENT ON COLUMN FREE_BOARD.create_time IS '생성일자';
COMMENT ON COLUMN FREE_BOARD.modify_time IS '수정일자';
COMMENT ON COLUMN FREE_BOARD.delete_time IS '삭제일자';

CREATE TABLE BOARD_COMMENT(
                              board_comment_id serial NOT NULL PRIMARY KEY,
                              free_board_id integer NOT NULL,
                              user_id integer NOT NULL,
                              contents text,
                              create_time timestamp,
                              modify_time timestamp,
                              delete_time timestamp
);
COMMENT ON TABLE BOARD_COMMENT IS '자유게시판 댓글';
COMMENT ON COLUMN BOARD_COMMENT.board_comment_id IS '댓글 ID';
COMMENT ON COLUMN BOARD_COMMENT.free_board_id IS '자유게시판 글 ID';
COMMENT ON COLUMN BOARD_COMMENT.user_id IS '사용자 ID';
COMMENT ON COLUMN BOARD_COMMENT.contents IS '댓글';
COMMENT ON COLUMN BOARD_COMMENT.create_time IS '생성일자';
COMMENT ON COLUMN BOARD_COMMENT.modify_time IS '수정일자';
COMMENT ON COLUMN BOARD_COMMENT.delete_time IS '삭제일자';