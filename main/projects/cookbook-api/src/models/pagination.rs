use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::AppError;

#[allow(dead_code)]
const MAX_PER_PAGE: i64 = 100;

#[derive(Debug, Deserialize, IntoParams)]
pub struct PaginationParams {
    /// 页码（从 1 开始）
    #[param(default = 1, minimum = 1)]
    pub page: i64,
    /// 每页数量（1-100）
    #[param(default = 20, minimum = 1, maximum = 100)]
    pub per_page: i64,
}

impl PaginationParams {
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), AppError> {
        if self.page < 1 {
            return Err(AppError::Validation("page must be >= 1".to_string()));
        }
        if self.per_page < 1 || self.per_page > 100 {
            return Err(AppError::Validation(
                "per_page must be between 1 and 100".to_string(),
            ));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn offset(&self) -> i64 { (self.page - 1) * self.per_page }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PagedResult<T: Serialize + for<'a> ToSchema<'a>> {
    pub data: Vec<T>,
    /// 当前页码
    pub page: i64,
    /// 每页数量
    pub per_page: i64,
    /// 总记录数
    pub total: i64,
    /// 总页数
    pub total_pages: i64,
}

impl<T: Serialize + for<'a> ToSchema<'a>> PagedResult<T> {
    pub fn new(data: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        let total_pages = (total + params.per_page - 1) / params.per_page;
        Self {
            data,
            page: params.page,
            per_page: params.per_page,
            total,
            total_pages,
        }
    }
}
