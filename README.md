# Web-Demo

一个基于 Axum + SeaORM 的 CRUD Demo 项目。

## 技术栈

| 组件 | 技术 |
|------|------|
| Web 框架 | Axum 0.8 |
| ORM | SeaORM 2.0 (PostgreSQL) |
| 缓存 | Redis (验证码存储、频率限制) |
| 异步运行时 | Tokio |
| 序列化 | Serde |
| 字段校验 | Validator |
| 认证 | JWT (access + refresh 双 token) |
| 密码/哈希 | bcrypt |
| OpenAPI 文档 | utoipa + utoipauto (自动收集) |
| 文档 UI | Scalar |

## 项目结构

```
src/
├── main.rs                 # 入口：服务器启动、中间件、路由挂载
├── api_doc.rs              # OpenAPI 文档定义（utoipauto 自动扫描）
├── constants.rs            # 全局常量（AppState、CONFIG）
├── helper/
│   ├── mod.rs
│   └── tools.rs            # 工具函数（日期格式化等）
├── apps/
│   ├── mod.rs              # 路由注册
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── view.rs         # 认证接口（发验证码、注册、登录、刷新 token）
│   │   ├── schemas.rs      # 认证请求/响应结构体
│   │   └── jwt.rs          # JWT 创建与验证
│   └── user/
│       ├── mod.rs
│       ├── view.rs         # 用户接口（CRUD）
│       ├── schemas.rs      # 用户请求/响应结构体
│       └── constants.rs    # 用户常量（枚举类型）
└── project/
    ├── mod.rs
    ├── configs.rs          # 配置加载（Jwt、Redis 等）
    ├── db.rs               # 数据库 + Redis 连接初始化
    ├── error.rs            # 统一错误处理
    ├── extractor.rs        # 自定义提取器（ResourceId）
    ├── logger.rs           # 日志系统
    ├── pagination.rs       # 分页结构体
    ├── redis.rs            # Redis 工具（频率限制宏）
    ├── response.rs         # 统一响应格式
    └── middlewares/
        ├── mod.rs
        ├── auth.rs         # JWT 认证中间件
        └── response.rs     # 响应中间件
```

## API 文档

项目使用 [utoipauto](https://crates.io/crates/utoipauto) 自动扫描 `#[utoipa::path]` 注解，无需手动注册路由。

启动后访问 Scalar UI：

```
http://localhost:8000/docs
```

### OpenAPI 规范说明

- `#[derive(IntoParams)]` 默认将参数标记为 `in = "path"`
- Query 参数需显式指定：`#[into_params(parameter_in = Query)]`
- `DateTimeLocal` 等自定义类型需用 `#[schema(value_type = String)]` 指定 OpenAPI 类型
- 所有响应体自动包装为 `{ code, message, data }` 结构（通过 `ApiDoc::spec()` 后处理）
- 全局 Bearer Auth 通过 `JwtSecurityAddon`（`Modify` trait）添加
- 公开接口使用 `security(())` 覆盖全局认证要求

## 运行

```bash
# 确保 PostgreSQL 和 Redis 已启动，config.toml 配置正确
cargo run
```
