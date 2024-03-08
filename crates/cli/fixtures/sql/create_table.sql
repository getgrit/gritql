Create table temp(
    first_name VARCHAR2(128),
    last_name VARCHAR2(128),
    empID NUMBER,
    salary NUMBER(6),
    pkey NUMBER(12,0),
    category_name VARCHAR2(15 char), 
	boid VARCHAR2(40 byte),
    info CLOB,
    data BLOB,
    pct_complete FLOAT,
	updated_at TIMESTAMP(9),
    XMLTYPE config
);
