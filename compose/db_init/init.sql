CREATE TABLE IF NOT EXISTS admins (
                id int NOT NULL AUTO_INCREMENT,
                username varchar(255) NOT NULL,
                password varchar(255) NOT NULL,
                created_at varchar(255) NOT NULL,
                PRIMARY KEY (id)
            );

CREATE TABLE IF NOT EXISTS users (
                id int NOT NULL AUTO_INCREMENT,
                username varchar(255) NOT NULL,
                password varchar(255) NOT NULL,
                created_at varchar(255) NOT NULL,
                PRIMARY KEY (id)
            );

CREATE TABLE IF NOT EXISTS shops (
                id int NOT NULL AUTO_INCREMENT,
                created_at varchar(255) NOT NULL,
                admin_id int NOT NULL,
                description varchar(255) NOT NULL,
                name varchar(255) NOT NULL,
                address varchar(255) NOT NULL,
                main_picture varchar(255) NOT NULL,
                phone varchar(255) NOT NULL,
                PRIMARY KEY (id)
            );

CREATE TABLE IF NOT EXISTS dishes (
                id int NOT NULL AUTO_INCREMENT,
                created_at varchar(255) NOT NULL,
                shop_id int NOT NULL,
                name varchar(255) NOT NULL,
                price int NOT NULL,
                description varchar(255) NOT NULL,
                picture varchar(255) NOT NULL,
                PRIMARY KEY (id)
            );