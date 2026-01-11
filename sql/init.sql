create table app
(
    id           bigint       not null
        primary key,
    tenant_id    bigint       not null comment '租户ID',
    workspace_id bigint       not null comment '工作空间ID',
    folder_id    bigint       not null comment '所属目录 ID',
    app_type     tinyint      not null comment '应用类型：1-工作流，2-对话流，3-智能体',
    name         varchar(50)  not null comment '应用名称',
    tags         json         null comment '标签列表',
    spec         text         null,
    description  varchar(200) null comment '应用描述',
    create_at    datetime     null comment '创建时间',
    update_at    datetime     null comment '更新时间'
)
    comment '应用';

create index idx_app_folder_type
    on app (tenant_id, workspace_id, app_type);

create table app_version
(
    id           bigint            not null
        primary key,
    tenant_id    bigint            not null comment '租户ID',
    workspace_id bigint            not null comment '工作空间ID',
    app_id       bigint            not null comment '应用ID',
    version      varchar(50)       not null comment '版本号（如 1.2.0-beta)',
    major        int               null comment '主版本号',
    minor        int               null comment '次版本号',
    patch        int               null comment '修订号',
    pre_release  varchar(20)       null comment '预发布标识，如 beta, rc1',
    spec         text              null,
    description  varchar(200)      null comment '版本说明',
    is_latest    tinyint default 0 not null comment '是否为最新版本',
    create_at    datetime          not null comment '创建时间'
)
    comment '应用版本';

create index idx_app_release_app_id
    on app_version (tenant_id, workspace_id, app_id);

create table user
(
    id          bigint               not null
        primary key,
    tenant_id   bigint               not null comment '租户id',
    name        varchar(20)          not null comment '用户名称',
    email       varchar(100)         not null comment '电子邮箱',
    passwd      varchar(40)          null comment '密码',
    is_owner    tinyint(1) default 0 not null comment '所有者',
    description varchar(100)         null comment '描述',
    create_at   datetime             not null comment '创建时间',
    update_at   datetime             not null comment '更新时间',
    constraint uk_user_email
        unique (email)
)
    comment '用户';

create table workspace
(
    id          bigint                             not null
        primary key,
    tenant_id   bigint                             not null comment '租户id',
    name        varchar(30)                        not null comment '工作空间名称',
    description varchar(200)                       null comment '描述',
    create_at   datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_at   datetime                           not null comment '更新时间'
)
    comment '工作空间';

create table workspace_folder
(
    id           bigint           not null
        primary key,
    tenant_id    bigint           not null comment '租户ID',
    workspace_id bigint           not null comment '工作空间 id',
    parent_id    bigint default 0 not null comment '上级id',
    folder_type  tinyint          not null comment '目录类型',
    name         varchar(50)      not null comment '目录名称',
    seq          int              null comment '序号',
    create_at    datetime         null comment '创建时间',
    update_at    datetime         null comment '更新时间'
)
    comment '应用目录';

create index idx_app_folder_parent_id
    on workspace_folder (parent_id);

create index idx_app_folder_workspace_id
    on workspace_folder (tenant_id);

create table workspace_tag
(
    id           bigint      not null comment 'pk'
        primary key,
    tenant_id    bigint      not null comment '租户Id',
    workspace_id bigint      not null comment '工作空间ID',
    name         varchar(50) not null comment '名称',
    tag_type     tinyint     null comment '标签类型：1-应用,2-知识库',
    create_at    datetime    not null comment '创建时间',
    update_at    datetime    not null comment '修改时间'
)
    comment '标签';