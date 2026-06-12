use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_auto(Account::Id))
                    .col(string_len_uniq(Account::Uid, 36))
                    .col(small_integer(Account::Type))
                    .col(string(Account::Nickname))
                    .col(string(Account::Password))
                    .col(string_null(Account::Name))
                    .col(small_integer_null(Account::Gender))
                    .col(string_len_null(Account::Telephone, 20))
                    .col(string_null(Account::Email))
                    .col(string_null(Account::Address))
                    .col(date_time_null(Account::LastLoginDt))
                    .col(date_time(Account::CreateTs).default(Expr::current_timestamp()))
                    .col(date_time(Account::UpdateTs).default(Expr::current_timestamp()))
                    .col(date_time_null(Account::DeleteTs))
                    .to_owned(),
            )
            .await?;

        // PostgreSQL 使用 COMMENT ON COLUMN 添加列注释
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            COMMENT ON COLUMN account.uid IS '账户唯一标识';
            COMMENT ON COLUMN account.type IS '账户类型;1:用户;2:内部人员';
            COMMENT ON COLUMN account.nickname IS '昵称';
            COMMENT ON COLUMN account.name IS '用户名';
            COMMENT ON COLUMN account.gender IS '性别;1:男;2:女';
            COMMENT ON COLUMN account.telephone IS '手机号码';
            COMMENT ON COLUMN account.email IS '邮件地址';
            COMMENT ON COLUMN account.address IS '居住地址';
            COMMENT ON COLUMN account.last_login_dt IS '最后登录时间';
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Uid,
    Type,
    Nickname,
    Password,
    Name,
    Gender,
    Telephone,
    Email,
    Address,
    LastLoginDt,
    CreateTs,
    UpdateTs,
    DeleteTs,
}
