#[derive(Debug)]
pub struct PaginatorPage<D> {
    pub data: Vec<D>,
    pub total_pages: usize,
    pub page_number: usize,
    pub per_page: usize,
}

impl<D> PaginatorPage<D> {
    pub fn new(data: Vec<D>, total_pages: usize, page: usize, per_page: usize) -> Self {
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

pub fn paginate<D>(data: &[D], per_page: usize) -> Vec<PaginatorPage<D>>
where
    D: Clone,
{
    let chunks = data.chunks(per_page);

    let total_chunks = chunks.len();

    chunks
        .enumerate()
        .map(|(i, chunk)| PaginatorPage {
            data: chunk.to_vec(),
            total_pages: total_chunks,
            page_number: i + 1,
            per_page,
        })
        .collect()
}
