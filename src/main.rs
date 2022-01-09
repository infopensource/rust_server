//#![deny(warnings)]
#![allow(non_snake_case)]
#![allow(unused)]

use warp::Filter;
use warp::body::json;
use warp::reply;

use serde_json::json;

// struct Article{
//     articleTitle: String,
//     articleText: String,
// }
// struct ArticleList {
//     articles: Vec<Article>,
// }

#[tokio::main]

async fn main() {
    println!("Welcome to little guy's sever console!");
    
    //FUNC展示主页
    //GET / -> Text
    //let show_HomePage = warp::fs::dir("../");
    //成功连接后发送一条欢迎信息
    let say_hello = 
        warp::path::end().map(|| {
            println!("Someone connected!");
            "Welcome to Little Guy's HomePage~
            try to visit ./HomePage.html".replace("    ", "")
        });

    //FUNC获取文章
    //GET /getArticles -> ArticleList
    let get_article = 
        warp::path("getArticles")
        .and(warp::path::param())//参数是文章分区
        .map(|article_partition: String| {
            format!("The article_partition you request is: {}", article_partition);
            //let article_list = ArticleList{articles: vec![Article{articleTitle:String::from("title1"), articleText: String::from("text1"})]};
            //let article_list = articles{vec![{articleTitle: "tt1"},]};
            let article_list = json!({
                "articles":[
                    {
                        "articleTitle": "tt1",
                        "articleText": "text1",
                    },
                    {
                        "articleTitle": "tt2",
                        "articleText": "text2",
                    }
                ]
            });
            warp::reply::json(&article_list)
        });

    //FUNC 获取个人简介
    //Get /getIntroduction -> Introduction
    let get_introduction = 
        warp::path("getIntroduction")
        .map(||{
            let introduction = json!({
                "introduction": {
                    "introduction": "This is Little Guy's introduction",
                }
            });
            warp::reply::json(&introduction)
        });

    //FUNC 获取头像
    //Get /getAvatarUrl -> AvataUrl
    let get_avatar = 
        warp::path("getAvatarUrl")
        .map(||{
            let avatar = json!({
                "avatar":{
                    "avatar":"/images/avatar/avatar.jpg"
                }
            });
            warp::reply::json(&avatar)
        });
        
    //FUNC 获取导航栏条目与其绑定的URL
    //GET /getNavItems -> navItems
    let get_navItems = 
        warp::path("getNavItems")
        .map(||{
            let navItems = json!({
                "navItems":[
                    {
                        "navItem":"item1",
                        "navItemUrl":"url1",
                    },
                    {
                        "navItem":"item2",
                        "navItemUrl":"url2",
                    },
                ]
            });
            warp::reply::json(&navItems)
        });

    let routes = 
        //show_HomePage
        say_hello
        .or(get_article)
        .or(get_introduction)
        .or(get_avatar)
        .or(get_navItems);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 80))
        .await;
}