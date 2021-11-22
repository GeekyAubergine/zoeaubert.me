export type MarkdownRemarkResponse = {
        frontmatter: {
            title: string,
            slug: string,
            categories: string[],
            description: string,
            date: string,
        },
        timeToRead: number,
        html: string,
}

export type MarkdownRemarkNode = {
    node: MarkdownRemarkResponse,
}

export type PhotoFile = {
    childImageSharp: {
        gatsbyImageData: {
            width: number,
            height: number,
        },
    }
}

export type PhotoResponse = {
    uid: string,
    smallPhoto: PhotoFile,
    largePhoto: PhotoFile,
}

export type PhotoNode = {
    node: PhotoResponse,
}