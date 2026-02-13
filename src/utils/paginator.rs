#[derive(Debug)]
pub struct PaginatorPage<'d, D> {
    pub data: &'d [D],
    pub total_pages: usize,
    pub page_number: usize,
    pub per_page: usize,
}

impl<'d, D> PaginatorPage<'d, D> {
    pub fn new(data: &'d [D], total_pages: usize, page: usize, per_page: usize) -> Self {
        Self {
            data,
            total_pages,
            page_number: page,
            per_page,
        }
    }

    pub fn has_next(&self) -> bool {
        self.page_number < self.total_pages
    }

    pub fn has_previous(&self) -> bool {
        self.page_number > 1
    }
}

pub fn paginate<'d, D>(data: &'d[D], per_page: usize) -> Vec<PaginatorPage<'d, D>> {
    let mut pages = Vec::new();

    let chunks = data.chunks(per_page);

    let total_chunks = chunks.len();

    for (page_number, chunk) in chunks.enumerate() {
        let page = PaginatorPage::new(chunk, total_chunks, page_number + 1, per_page);
        pages.push(page);
    }

    pages
}
