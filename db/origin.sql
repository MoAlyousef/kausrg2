PRAGMA encoding="UTF-8";

CREATE TABLE "staff" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"namen"	TEXT UNIQUE,
    "namar"	TEXT UNIQUE,
	"nat"	INTEGER,
	"rank"	INTEGER,
	"div"	INTEGER,
	"spec"	TEXT,
    "specr" TEXT,
	"qual"	TEXT,
    "qlar"  TEXT,
	"email"	TEXT,
	"image" TEXT
);

-- rank 1 prof, 2 assoc, 3 assist
-- nat 11 2 egy
-- div:
-- GS 1
-- NS 2
-- CS 3
-- TS 4
-- VS 5
-- pls 6
-- peds 7

INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (1, 'Osama Mohammed Rayyes', 'أسامة محمد ريس', 1, 1, 7);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (2, 'Iskandar Sulaiman Alqethmi', 'اسكندر سليمان القثمي', 1, 1, 4);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (3, 'Jamal Siddiq Kamal', 'جمال صديق كمال', 1, 1, 7);
-- INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (4, 'Hassan Ali Alzahrani', 'حسن علي الزهراني', 1, 1, 5);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (4, 'Hussein Hamza Jabbad', 'حسين حمزة جباد', 1, 1, 3);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (5, 'Khaled Ibraheem Al-Ibraheem', 'خالد إبراهيم آل إبراهيم', 1, 1, 3);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (6, 'Zuhoor Khader Alghaithi', 'زهور خضر الغيثي الشريف', 1, 1, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (7, 'Saleh Salem Baeesa', 'صالح سالم باعيسى', 1, 1, 2);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (8, 'Saleh Mohammed Aldaqal', 'صالح محمد الدقل', 1, 1, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (9, 'Sabah Saleh Moshref', 'صباح صالح مشرف', 1, 1, 6);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (10, 'Abdulrahman Mohammed Sebyani', 'عبدالرحمن محمد صبياني', 1, 1, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (11, 'Adnan Abdulmuti Merdad', 'عدنان عبد المعطي مرداد', 1, 1, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (12, 'Faisal Mohammed Almashat', 'فيصل محمد حسين المشاط', 1, 1, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (13, 'Mohammed Ahmad Alharbi', 'محمد أحمد الحربي', 1, 1, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (14, 'Mohammed Hassan Bangash', 'محمد حسن بنقش', 1, 1, 2);
-- INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (17, 'Yaser Saleh Jamal', 'ياسر صالح جمال', 1, 1, 7);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (15, 'Bassam Mohammed Addas', 'بسام محمد عداس', 1, 2, 2);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (16, 'Adel Ali Ajohari', 'عادل علي الجوهري', 1, 2, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (17, 'Abdulmalek Mohammed Altaf', 'عبدالملك محمد ألطف', 1, 2, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (18, 'Othman Osama Radi', 'عثمان أسامة الراضي', 1, 2, 3);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (19, 'Fatimah Khunayfis Althobaiti', 'فاطمة خنيفس الثبيتي', 1, 2, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (20, 'Mazen Omar Kordi', 'مازن عمر كردي', 1, 2, 7);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (21, 'Morad Mustafa Aljefri', 'مراد مصطفى الجفري', 1, 2, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (22, 'Munassar Saleh Alamoudi', 'منصر صالح العمودي', 1, 2, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (23, 'Ahmad Mohammed Makki', 'أحمد محمد مكي', 2, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (24, 'Ashraf Abdulrahman Maghrabi', 'أشرف عبدالرحمن مغربي', 1, 3, 4);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (25, 'Basem Abdullah Awan', 'باسم عبدالله أوان', 1, 3, 6);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (26, 'Hatim Ali Al-abbadi', 'حاتم علي العبادي', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (27, 'Reda Abdullah Jamjoom', 'رضا عبدالله  جمجوم', 1, 3, 5);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (28, 'Soha Abdo Alomar', 'سهى عبده آل عمر', 1, 3, 2);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (29, 'Abdulrahman Jafar Sabbagh', 'عبدالرحمن جعفر صباغ', 1, 3, 2);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (30, 'Abdulaziz Mamdouh Saleem', 'عبدالعزيز ممدوح سليم', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (31, 'Ali Hasan Farsi', 'علي حسن فارسي', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (32, 'Ali Abdullah Samkari', 'علي عبدالله سمكري', 1, 3, 1);
-- INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (38, 'Majed Mahmoud Mansouri', 'ماجد محمود منصوري', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (33, 'Mohammed Ahmed Ghunaim', 'محمد أحمد غنيم', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (34, 'Mohammed Osama Nassif', 'محمد أسامة ناصف', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (35, 'Mohammed Hussain Basondowah', 'محمد حسين باسندوة', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (36, 'Mohammed Hammad Alharthi', 'محمد حماد الحارثي', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (37, 'Mohammed Abdulaziz Alyousef', 'محمد عبدالعزيز اليوسف', 1, 3, 2);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (38, 'Maram Taha Alkhateeb', 'مرام طه الخطيب', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (39, 'Moaz Waleed Abualfaraj', 'معاذ وليد أبو الفرج', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (40, 'Nasser Mohammed Bustanji', 'ناصر محمد بستنجي', 1, 3, 7);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (41, 'Nadim Malibary', 'نديم حسين مليباري', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (42, 'Nora Hatim Tarabulsi', 'نورة حاتم طرابلسي', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (43, 'Nouf Yihya Aqeel', 'نوف يحيى عقيل', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (44, 'Hattan Abdulhafidh Aljaali', 'هتان عبدالحافظ الجعلي', 1, 3, 6);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (45, 'Hanaa Mohammed Tashkandi', 'هناء محمد طاشكندي', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (46, 'Wael Abduhafeez Tashkandi', 'وائل عبدالحفيظ طاشكندى', 1, 3, 1);
INSERT INTO staff (id, namen, namar, nat, rank, div) VALUES (47, 'Yehya Abdullah Almarhabi', 'يحي عبدالله المرحبي', 1, 3, 1);

UPDATE staff SET spec='Neurosurgery and skull base neurosurgery' WHERE id=37;