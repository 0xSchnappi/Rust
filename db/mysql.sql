create database hillhouse default character set utf8mb4 collate utf8mb4_unicode_ci;
create user 'username'@'localhost' identified by 'password';
GRANT ALL ON hillhouse.* TO 'username'@'localhost';