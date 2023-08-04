<h3 align="center">TODO backend</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/kylelobo/The-Documentation-Compendium.svg)](https://github.com/higoraln/todo_backend_rust/pulls)

</div>

---

<p>
This project is a TODO backend, it's a REST API that allows you to manage your tasks.
It's a project I'm doing to learn more about backend and rust. I'll be using the rocket framework, but it could be any other, nothing personal with rocket.
</p>

## 📝 Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Built Using](#built_using)
- [Authors](#authors)
- [Acknowledgments](#backlog)

## 🧐 About <a name = "about"></a>

In this api, you can create your account, login to the system, create your categories, and create tasks within the category. In some routes, it is necessary to pass the "Authorization" in the header with the "Bearer {Token}". This api is totally free some examples of use: Test call to some api in a language study, use it in someone's study app.

## 🏁 Getting Started <a name = "getting_started"></a>

you can run this project with docker, just run the command below in the root folder of the project.

### Prerequisites

What things you need to install the software and how to install them.

```
rustup
```

### Installing

A step by step series of examples that tell you how to get a development env running.

Say what the step will be get URI to connect to the database and put in .env file

```
cargo update
```
```
cargo run
```

End with an example of getting some data out of the system or using it for a little demo.


## 🎈 Usage <a name="usage"></a>

if you like to use docker, you can run the command below to create the image and run the container.

don't forget to change the .env file with your database uri.

## ⛏️ Built Using <a name = "built_using"></a>

- [MongoDB](https://www.mongodb.com/) - Database
- [Rust](https://www.rust-lang.org/) - Language
- [Rocket](https://rocket.rs/) - Framework server

## ✍️ Authors <a name = "authors"></a>

- [@HigorAllan](https://github.com/higoraln) - Idea & Initial work

## 🎉 Backlog <a name = "backlog"></a>

- [] Create tests
- [] Create a dockerfile
- [] Create a documentation
- [] Create a frontend
