use std::mem::take;

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

pub trait Paginator<D>
where
    D: Clone,
{
    fn paginate(&mut self, per_page: usize) -> impl Iterator<Item = PaginatorPage<D>>;
}

impl<D, I> Paginator<D> for I
where
    D: Clone,
    I: Iterator<Item = D>,
{
    fn paginate(&mut self, per_page: usize) -> impl Iterator<Item = PaginatorPage<D>> {
        let mut chunks: Vec<Vec<D>> = vec![];

        let mut chunk: Vec<D> = Vec::with_capacity(per_page);

        for item in self {
            chunk.push(item);

            if chunk.len() == per_page {
                chunks.push(take(&mut chunk));
            }
        }

        if !chunk.is_empty() {
            chunks.push(chunk);
        }

        let total_chunks = chunks.len();

        chunks
            .into_iter()
            .enumerate()
            .map(move |(i, chunk)| PaginatorPage {
                data: chunk.to_vec(),
                total_pages: total_chunks,
                page_number: i + 1,
                per_page,
            })
    }
}
