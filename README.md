# SimpleDB Based on Rust

## 开始使用

1. 安装环境
2. xxx

## 系统命令

在使用系统命令时，除**登录**外，命令格式为`sys <command> <query> <others>`。

### 用户管理

1. 登录

命令原型：`<username> <password>`

2. 改密

命令原型：`sys changepwd <new_pwd>`

### 数据库管理

#### 新建数据库

命令原型：`sys createdb <db_name>`

Test Command:

```sql
CREATE TABLE users
(
    id       INT PRIMARY KEY,
    username VARCHAR(50),
    password VARCHAR(50),
    email    VARCHAR(100)
);
```

#### 切换数据库

命令原型：`sys usedb <db_name>`

#### 删除数据库

命令原型：`sys dropdb <db_name>`

#### 展示数据库

命令原型：`sys showdb <db_name>`

### 系统管理

#### 系统信息

命令原型：`sys showsys`

## 数据表命令

### 新建数据表

### 插入数据

```sql
INSERT INTO users (id, username, password, email)
VALUES (1, 'user1', 'password123', 'user1@example.com'),
       (2, 'user2', 'pass456', 'user2@example.com'),
       (3, 'user3', 'secret', 'user3@example.com');
```

### 选取数据

```sql
SELECT id, username, password, email
FROM users;
```

支持多条件查询

### TODO
- 多表查询
- 在表里查表
- update
- join
- primary key