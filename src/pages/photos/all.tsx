import * as React from 'react'
import {
    Photo as PhotoType,
    ALBUMS_BY_DATE,
} from '../../../res/photos'
import { Page } from '../../components/ui/Page'
import PhotoGrid from '../../components/ui/PhotoGrid'

export default function AllPhotos() {
    const allPhotos: PhotoType[] = React.useMemo(
        () =>
            ALBUMS_BY_DATE.reduce(
                (acc: PhotoType[], album) => acc.concat(album.photos),
                [],
            ),
        [],
    )

    return (
        <Page title="Photos">
            <h2 className="text-2xl pt-12 mb-2 font-bold sm:pt-8">
                All Photos
            </h2>
            <PhotoGrid photos={allPhotos} className="mb-8" />
        </Page>
    )
}
