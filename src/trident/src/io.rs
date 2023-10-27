// 输出使用 pretty 下输出使用 json 包装结构体

// {
//    "result": "success" | "fail" | "not found",
//    "data": { ... } // 如果是 fail 里面就是 error 信息， 必定是 object，需要传 array 就外面包一层
//    "echo": { ... } | null
// }

// 像还原是异步输出，就输出多个这样的结构体，其中 echo 就的形式就具有以下形式 { "type": "component", "id": "net.minecraft", "version": "1.20.1" }
// 这些字段必定出现，但内容依赖其他字段的值，例如 "not found" 时 data 就必定为 {}，对于不同的命令，有些就不需要 echo 字段，其值为 Null