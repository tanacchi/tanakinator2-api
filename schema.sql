DROP DATABASE tanakinator2 IF EXISTS;
CREATE DATABASE tanakinator2 CHARACTER SET utf8;
USE tanakinator2;

CREATE TABLE question (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    body TEXT
);

INSERT INTO question(body) VALUES("Question 1");
INSERT INTO question(body) VALUES("Question 2");
INSERT INTO question(body) VALUES("Question 3");
