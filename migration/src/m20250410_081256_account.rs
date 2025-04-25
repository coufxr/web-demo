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
                    .col(string_len_uniq(Account::Uid, 36).extra("COMMENT '账户唯一标识'"))
                    .col(tiny_unsigned(Account::Type).extra("COMMENT '账户类型;1:用户;2:内部人员'"))
                    .col(string(Account::Nickname).extra("COMMENT '昵称'"))
                    .col(string(Account::Password))
                    .col(string_null(Account::Name).extra("COMMENT '用户名'"))
                    .col(tiny_unsigned_null(Account::Gender).extra("COMMENT '性别;1:男;2:女'"))
                    .col(string_len_null(Account::Telephone, 20).extra("COMMENT '手机号码'"))
                    .col(string_null(Account::Email).extra("COMMENT '邮件地址'"))
                    .col(string_null(Account::Address).extra("COMMENT '居住地址'"))
                    .col(date_time_null(Account::LastLoginDt).extra("COMMENT '最后登录时间'"))
                    .col(date_time(Account::CreateTs).default(Expr::current_timestamp()))
                    .col(date_time(Account::UpdateTs).default(Expr::current_timestamp()))
                    .col(date_time_null(Account::DeleteTs))
                    .to_owned(),
            )
            .await
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
