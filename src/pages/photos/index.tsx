import React, { useCallback } from 'react'
import { ALBUM_YEARS } from '../../../res/photos'
import { Page } from '../../components/ui/Page'
import { graphql, Link, useStaticQuery } from 'gatsby'
import AlbumsYearGroup from '../../components/ui/AlbumsYearGroup'
import { usePhotoNodeData } from '../../utils'

const MAX_FEATURED_PHOTOS = 9

export default function IndexPage() {
    // const featuredPhotos: PhotoType[] = React.useMemo(() => {
    //     return ALBUMS_BY_DATE.reduce(
    //         (acc: PhotoType[], album) =>
    //             acc.concat(album.photos.filter((photo) => photo.featured)),
    //         [],
    //     ).slice(0, MAX_FEATURED_PHOTOS)
    // }, [])

    // const { onPhotoClick, Component: PhotoViewerComponent } = usePhotoViewer({
    //     photos: featuredPhotos,
    // })

    // const onClickCallback = React.useCallback(
    //     (photo: PhotoType) => {
    //         onPhotoClick(photo)
    //     },
    //     [onPhotoClick],
    // )
    const photoNodeData = usePhotoNodeData()

    const renderYear = useCallback(
        (year) => (
            <AlbumsYearGroup
                key={year}
                year={year}
                photoNodeData={photoNodeData}
            />
        ),
        [photoNodeData],
    )

    return (
        <Page title="Photos">
            <div className="flex justify-between items-baseline">
                <h2 className="pageTitle">Photos</h2>
                <div>
                    <Link to="/photos/tags" className="link mr-4">
                        Tags
                    </Link>
                    <Link to="/photos/all" className="link">
                        All Photos
                    </Link>
                </div>
            </div>
            {/* {featuredPhotos.length > 0 && (
                <PhotoGrid
                    photos={featuredPhotos}
                    className="mb-8"
                    onClick={onClickCallback}
                />
            )} */}
            {ALBUM_YEARS.map(renderYear)}
            {/* {PhotoViewerComponent} */}
        </Page>
    )
}
