CREATE TABLE FARM(
                     farm_id serial NOT NULL PRIMARY KEY,
                     farm_name varchar(50) NOT NULL,
                     farm_address varchar(50) NOT NULL,
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

-- TEST DATA
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('강서구 오곡 텃밭','서울 강서구 오곡동 560-1',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('과해 주말 농장','서울 강서구 오곡동 518-2',1,null,'010-3720-9520',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('강서구 주말농장','서울 강서구 오곡동 417-2',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('행복한 도시농장','서울 강서구 오쇠동 151-11',1,null,'010-3380-4536',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('구로 주말농장','서울 구로구 궁동 17-10',1,null,'02-860-2312','60,000 (1개당)',0.0,null,null,false,'구로 구민만 가능, 신청기간 내 신청 후 일정 기간동안 이용 가능');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('문래동 공공공지 도시텃밭','영등포구 문래동 3가 55-6',1,null,'02-2670-3418','무료',0.0,null,null,false,'영등포구민만 가능, 주차장 없음, 친환경 원칙 준수, 사용 가능 시간 [4-9월 : 07:00~19:00, 10~11월 : 07:30~17:30]');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('금천구친환경주말농장주말농장','서울 금천구 시흥동 113-121',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('강감찬 텃밭','서울 관악구 봉천동 259-1',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('낙성대 공원 텃밭','서울 관악구 봉천동 231-2',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('다다연','서울 강남구 언주로 508 상록회관',1,null,'02-554-2298',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('대원주말농장','서울 서초구 원지동 227',1,null,'010-5497-4187',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('염곡약수터주말농장','서울 서초구 염곡동 33',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('들꽃풍경','서울 서초구 내곡동 1-336',1,null,'02-451-7700',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('서초구친환경도시텃밭','서울 서초구 신원동 225',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('청계주말농장','서울 서초구 원지동 532',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('율현 주말농장','서울 강남구 율현동 129-8',1,null,'0507-1431-0512',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('다다연','서울 강남구 언주로 508 상록회관',1,null,'02-554-2298',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('프랑베베농장','서울 송파구 동남로 371',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('태현주말농장','서울 송파구 방이동 436-52',1,null,'010-2251-9406',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('우리주말농장','서울 송파구 방이동 435-22',1,null,'0507-1307-4962',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('친환경 도시텃밭','서울 강동구 성내로 19',1,null,'02-3425-6550','70,000 (1개당)',0.0,null,null,false,'7개의 텃밭을 분배하는 것으로 확인됨');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('행복주말농장','서울 강동구 암사동 596-1',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('명일근린공원 공동체텃밭','서울 강동구 상일동 145-6',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('성일도시텃밭','서울 강동구 상일동 12-4',1,null,'0507-1481-5159',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('고덕주말농장','서울 강동구 고덕 1동 479',1,'어인경','010-4214-9347','15m^2 에 10만원',0.0,null,null,false,'출처 : https://agro.seoul.go.kr/archives/1289');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('둔촌텃밭농원','서울 강동구 둔촌동 125-1',1,'유병연','010-8484-3909','16m^2 에 12만원',0.0,null,null,false,'출처 : https://agro.seoul.go.kr/archives/1289');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('암사가족농원','서울 강동구 암사동 380-2,5~9',1,'유병연','010-8484-3909','16m^2 에 12만원',0.0,null,null,false,'출처 : https://agro.seoul.go.kr/archives/1289');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('토끼굴주말농장','서울 강동구 암사동 603-5',1,'김동철','010-8783-4520','12m^2 에 10만원',0.0,null,null,false,'출처 : https://agro.seoul.go.kr/archives/1289');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('강일텃밭 (강일 정원형텃밭)','서울 강동구 강일동 33-3 외 8',1,null,null,null,0.0,null,null,false,'출처 : https://cityfarm.gangdong.go.kr/site/main/program/programSelect');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('가래 여울 텃밭','서울 강동구 강일동 138-21',1,null,null,null,0.0,null,null,false,'출처 : https://cityfarm.gangdong.go.kr/site/main/program/programSelect');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('길동 텃밭','서울 강동구 길동 36-2',1,null,null,null,0.0,null,null,false,'출처 : https://cityfarm.gangdong.go.kr/site/main/program/programSelect');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('상일 텃밭','서울 강동구 상일동 12 외 1',1,null,null,null,0.0,null,null,false,'출처 : https://cityfarm.gangdong.go.kr/site/main/program/programSelect');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('암사 텃밭','서울 강동구 암사동 603-3',1,null,null,null,0.0,null,null,false,'출처 : https://cityfarm.gangdong.go.kr/site/main/program/programSelect');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('일자산 텃밭','서울 강동구 둔촌동 599',1,null,null,null,0.0,null,null,false,'출처 : https://cityfarm.gangdong.go.kr/site/main/program/programSelect');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('사가정주말농장텃밭','서울 중랑구 용마산로70길 81',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('중랑구친환경마을텃밭','서울 중랑구 묵1동 120-6',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('석관동도시텃밭','서울 성북구 석관동 14-5',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('경춘선숲길텃밭','서울 노원구 공릉동 272-2',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('이화농원','서울 중랑구 신내로15길 143',1,null,'0507-1412-1696',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('봉화골 이화팜','서울 중랑구 신내로21길 116-11 봉화골 이화팜',1,null,'0507-1432-0009',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('천수주말농장','서울 노원구 중계로8길 56',1,null,'02-930-5700',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('초안산근린공원나눔텃밭','서울 도봉구 해등로 32',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('무수골주말농장','서울 도봉구 도봉1동 469',1,null,'02-954-0329',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('초록향기주말농장','서울 도봉구 도봉1동 384-1',1,null,'031-383-5364',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('도봉산초원주말농장','서울 도봉구 평화로15번길 9-23 초원주말농장',1,null,'010-6255-1829',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('정릉동도시텃밭','서울 성북구 정릉동 908-4',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('팜스테이마을','서울 중구 새문안로 16 농협중앙회 중앙본부',1,null,'02-2080-5588',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('도시농부무료체험센터','서울 종로구 부암동 127-4',1,null,'0507-1360-1805',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('홍씨네 텃밭 농원','서울 종로구 부암동 353-1',1,'홍승규','010-2972-1595','10m^2 에 20만원',0.0,'06:00:00','19:00:00',false,'출처 : https://agro.seoul.go.kr/archives/1289');
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('향림도시농업체험원','서울 은평구 불광동 457',1,null,'02-382-8001',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('솔밭농장','서울 은평구 진관동 272-3',1,null,'010-6209-4757',null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('무지개텃밭','서울 성동구 행당동 76-3',1,null,null,null,0.0,null,null,false,null);
INSERT INTO farm(farm_name, farm_address, farm_address_div, farm_owner_name, farm_owner_phone, price, stars, available_use_start, available_use_end, available_lesson, etc) VALUES ('양천 도시 농업 공원','서울특별시 양천구 신월7동',1,null,' 02-2620-3578',null,0.0,null,null,false,null);

INSERT INTO farm_urls(farm_id, url_division, url) VALUES ((SELECT farm_id FROM farm WHERE farm_name = '구로 주말농장'),'ETC','https://cityfarmer.seoul.go.kr/ntcn/www/view.do?ntcnSn=2202&ntcnSeCode=NGC002&key=1905228762688&pageIndex=1&sc=&sw=%EA%B5%AC%EB%A1%9C%EA%B5%AC');
INSERT INTO farm_urls(farm_id, url_division, url) VALUES ((SELECT farm_id FROM farm WHERE farm_name = '문래동 공공공지 도시텃밭'),'ETC','https://cityfarmer.seoul.go.kr/ntcn/www/view.do?ntcnSn=2203&ntcnSeCode=NGC002&key=1905228762688&pageIndex=1&sc=&sw=%EA%B5%AC%EB%A1%9C');
INSERT INTO farm_urls(farm_id, url_division, url) VALUES ((SELECT farm_id FROM farm WHERE farm_name = '우리주말농장'),'ETC','http://woorifarmer435.modoo.at');
INSERT INTO farm_urls(farm_id, url_division, url) VALUES ((SELECT farm_id FROM farm WHERE farm_name = '친환경 도시텃밭'),'ETC','https://cityfarmer.seoul.go.kr/ntcn/www/view.do?ntcnSn=1421&ntcnSeCode=NGC002&key=1905228762688&pageIndex=1&sc=&sw=%EA%B0%95%EB%8F%99');
INSERT INTO farm_urls(farm_id, url_division, url) VALUES ((SELECT farm_id FROM farm WHERE farm_name = '무수골주말농장'),'ETC','http://hanhome.jinbo.net/');
INSERT INTO farm_urls(farm_id, url_division, url) VALUES ((SELECT farm_id FROM farm WHERE farm_name = '도봉산초원주말농장'),'ETC','https://blog.naver.com/limte3000');
INSERT INTO farm_urls(farm_id, url_division, url) VALUES ((SELECT farm_id FROM farm WHERE farm_name = '팜스테이마을'),'ETC','http://www.farmstay.co.kr/');
INSERT INTO farm_urls(farm_id, url_division, url) VALUES ((SELECT farm_id FROM farm WHERE farm_name = '홍씨네 텃밭 농원'),'ETC','https://hongsfarm.modoo.at/?pc=1');