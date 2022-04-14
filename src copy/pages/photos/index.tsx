import { graphql } from 'gatsby'
import AlbumsPage from '../../components/album/AlbumsPage'

export default AlbumsPage

export const pageQuery = graphql`
    query PhotoPageQuery {
        allPhoto {
            edges {
                node {
                    uid
                    alt
                    thumb: localFile {
                        childImageSharp {
                            gatsbyImageData(height: 200, layout: CONSTRAINED)
                        }
                    }
                }
            }
        }
        allAlbum {
            edges {
                node {
                    coords {
                        lat
                        lng
                    }
                    date(formatString: "YYYY-MM-DD")
                    description
                    location
                    photo_uids
                    slug
                    uid
                    cover_photo_uids
                    name
                }
            }
        }
    }
`
