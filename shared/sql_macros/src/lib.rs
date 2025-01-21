extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::{Data, parse_macro_input, DeriveInput, Field, Fields, Type, Ident};
use quote::quote;

enum SqlTypes {
    String,
    Uuid,
    Integer
}

impl SqlTypes {
    fn from_field(field: &Field) -> SqlTypes{
        let field_type = &field.ty;
        match field_type {
            Type::Path(type_path) => {
                let type_name = type_path.path.segments.first().unwrap().ident.to_string();
                match type_name.as_str() {
                    "String" => SqlTypes::String,
                    "i32" => SqlTypes::Integer,
                    "Uuid" => SqlTypes::Uuid,
                    _ => unimplemented!("Unimplemented type {}", type_name)
                }
            }
            _ => unimplemented!("Unkown type is not implemented"),
        }

    }

    fn to_sql_syntax(&self) -> &str {
        match self {
            SqlTypes::String => "VARCHAR(255)",
            SqlTypes::Integer => "INTEGER",
            SqlTypes::Uuid => "UUID",
        }
    }

    fn to_awsdata(&self, field_name: &&Ident) -> proc_macro2::TokenStream {
        match self {
            SqlTypes::String | SqlTypes::Uuid => quote!(aws_sdk_rdsdata::types::Field::StringValue(self.#field_name.to_string().clone())),
            SqlTypes::Integer => quote!(aws_sdk_rdsdata::types::Field::LongValue(self.#field_name.clone().into())),
        }
    }

    fn to_typehint(&self) -> proc_macro2::TokenStream {
        match self {
            SqlTypes::Uuid => quote!(Some(aws_sdk_rdsdata::types::TypeHint::Uuid)),
            _ => quote!(None)
        }
    }
}



/// Generate UPDATE ROW query
/// TODO: Manage other ids than uuid
fn update_row_query(fields: &Fields, struct_name: &Ident) -> String{
    let mut fields_sql = Vec::new();
    for field in fields {
        let field_name = &field.ident;

        if *field_name.as_ref().unwrap() != "uuid" {
            fields_sql.push(format!("{} = :{}", field_name.as_ref().unwrap(), field_name.as_ref().unwrap()));
        }

    };
    format!("UPDATE {} SET {} WHERE uuid = :uuid", struct_name, &fields_sql.join(", "))
}

/// Generate INSERT ROW query
fn insert_row_query(fields: &Fields, struct_name: &Ident) -> String{
    let mut fields_sql1 = Vec::new();
    let mut fields_sql2 = Vec::new();
    for field in fields {
        let field_name = &field.ident;

        fields_sql1.push(format!("{}", field_name.as_ref().unwrap()));
        fields_sql2.push(format!(":{}", field_name.as_ref().unwrap()));

    };
    format!("INSERT INTO {} ({}) VALUES ({})", struct_name, &fields_sql1.join(", "), &fields_sql2.join(", "))
}

/// Generate CREATE TABLE query
fn create_table_query(fields: &Fields, struct_name: &Ident) -> String{
    let mut fields_sql = Vec::new();
    for field in fields {
        let field_name = &field.ident;
        let sql_type = SqlTypes::from_field(field);

        fields_sql.push(format!("{} {}", field_name.as_ref().unwrap(), sql_type.to_sql_syntax()));

    };
    format!("CREATE TABLE IF NOT EXISTS {} ({})", struct_name, &fields_sql.join(", "))
}

/// Generate SqlParameters from the fields
fn fields_as_params(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let mut fields_params = Vec::new();

    for field in fields {
        let field_name = &field.ident.as_ref().unwrap();
        let field_name_as_string = field_name.to_owned().to_string();
        let sql_type = SqlTypes::from_field(field);

        // let value = match sql_type {
        //     Sq
        // };
        let value = sql_type.to_awsdata(field_name); 
        let type_hint = sql_type.to_typehint();

        fields_params.push(quote!{
            aws_sdk_rdsdata::types::SqlParameter::builder()
                .name(#field_name_as_string.to_string())
                .value(#value)
                .set_type_hint(#type_hint)
                .build()
        });
    }
    fields_params
}

#[proc_macro_attribute]
/// The macro attribute `struct_to_sql` enriches a struct 
/// to dynamically create sql queries
pub fn struct_to_sql(_metadata: TokenStream, item: TokenStream) -> TokenStream {
        let input = parse_macro_input!(item as DeriveInput);
        let struct_name = &input.ident; // The name of the struct will be used a the name of the table

        // Clone the fields
        let fields = match input.data {
            Data::Struct(ref data) => &data.fields,
            _ => unimplemented!("Only structs are supported"),
        };

        // Generate the fields for the new struct
        let field_defs = fields.iter().map(|field: &Field|{
            let field_name = &field.ident;
            let field_type = &field.ty;
            quote! {
                pub #field_name: #field_type,
            }
        });

        // Create table query
        let create_table_sql = create_table_query(fields, struct_name);

        // Insert row query
        let insert_row_sql = insert_row_query(fields, struct_name);

        // Update row query
        let update_row_sql = update_row_query(fields, struct_name);

        // Fiels as params
        let fap = fields_as_params(fields);

        // Generate methods for the new struct
        let mut methods = Vec::new();
        methods.push(quote!(
            /// The name of the struct is the name of the table
            fn table(&self) -> String {
                stringify!(#struct_name).to_string()
            }

            /// SQL query to create a new table
            fn create_table(&self) -> String {
                #create_table_sql.to_string()
            }

            /// SQL query to drop a table
            fn drop_table(&self) -> String {
                format!("DROP TABLE IF EXISTS {}", stringify!(#struct_name)).to_string()
            }
            
            /// SQL query to delete an object by field (prepared)
            fn delete(&self, field_name: &str) -> String {
                format!("DELETE FROM {} WHERE {} = :{}", stringify!(#struct_name), field_name, field_name).to_string()
            }

            /// SQL query to get an object by field (prepared)
            fn get(&self, field_name: &str) -> String{
                format!("SELECT * FROM {} WHERE {} = :{}", stringify!(#struct_name), field_name, field_name).to_string()
            }
            
            /// SQL query to create an object (prepared)
            fn create(&self) -> String{
                #insert_row_sql.to_string()
            }

            /// SQL query to update an object (prepared)
            fn update(&self) -> String {
                #update_row_sql.to_string()
            }

            /// SQL query to list all items
            fn list(&self) -> String {
                format!("SELECT * FROM {}", stringify!(#struct_name)).to_string()
            }


        ));

        let queryset_name = Ident::new(format!("{}QuerySet", struct_name).as_str(), proc_macro2::Span::call_site());
        quote! {
            // Don't modify the struct's fields
            #[derive(Clone, Debug, Default)]
            struct #struct_name {
                #(#field_defs)*
            }

            // Define a queryset for the model
            struct #queryset_name<#struct_name> {
                _struct: std::marker::PhantomData<#struct_name>
            }

            // Implement database calls
            impl<#struct_name> QuerySet<#struct_name> for #queryset_name<#struct_name> {
                #(#methods)*
            }

            // Define a constructor for a query set
            // from the model
            impl #struct_name {
                pub fn queryset() -> #queryset_name<#struct_name> {
                    #queryset_name {
                        _struct: std::marker::PhantomData
                    }
                }
            }

            /// Build a Vec<SqlParameter> to use in ExecuteStatementBuilder::set_parameters.
            /// from an items fields
            impl GetFieldsAsParams for #struct_name {

                fn get_fields_as_params(&self) -> Option<Vec<aws_sdk_rdsdata::types::SqlParameter>> {
                    Some(vec![
                        #(#fap),*
                    ])
                }
            }

            // /// Get the primary key as a param
            // fn get_uuid_as_param(&self) -> Option<aws_sdk_rdsdata::types::SqlParameter>{
            //     Some(
            //         aws_sdk_rdsdata::types::SqlParameter::builder()
            //             .name("uuid".to_string())
            //             .value(aws_sdk_rdsdata::types::Field::StringValue(self.uuid.to_string().clone()))
            //             .build()
            //     )
            // }


        }.into()
}
