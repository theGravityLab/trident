// 输出使用 pretty 下输出使用 json 包装结构体

// {
//    "success": true | false,
//    "data": { ... } // 如果是 fail 里面就是 error 信息， 必定是 object，需要传 array 就外面包一层
//    "echo": { ... } | null
// }

// 像还原是异步输出，就输出多个这样的结构体，其中 echo 就的形式就具有以下形式 { "type": "component", "id": "net.minecraft", "version": "1.20.1" }

// 先别考虑同时兼容问题，先想一下如何 cli 中更好的展示异步下载进度之类的效果
// 先实现非 pretty 的