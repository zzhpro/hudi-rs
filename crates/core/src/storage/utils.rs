/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use std::path::Path;

use anyhow::{anyhow, Result};
use url::{ParseError, Url};

pub fn split_filename(filename: &str) -> Result<(String, String)> {
    let path = Path::new(filename);

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("No file stem found"))?
        .to_string();

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_string();

    Ok((stem, extension))
}

pub fn join_url_segments(base_url: &Url, segments: &[&str]) -> Result<Url> {
    let mut url = base_url.clone();

    if url.path().ends_with('/') {
        url.path_segments_mut().unwrap().pop();
    }

    for &seg in segments {
        let segs: Vec<_> = seg.split('/').filter(|&s| !s.is_empty()).collect();
        url.path_segments_mut()
            .map_err(|_| ParseError::RelativeUrlWithoutBase)?
            .extend(segs);
    }

    Ok(url)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use url::Url;

    use crate::storage::utils::join_url_segments;

    #[test]
    fn join_base_url_with_segments() {
        let base_url = Url::from_str("file:///base").unwrap();

        assert_eq!(
            join_url_segments(&base_url, &["foo"]).unwrap(),
            Url::from_str("file:///base/foo").unwrap()
        );

        assert_eq!(
            join_url_segments(&base_url, &["/foo"]).unwrap(),
            Url::from_str("file:///base/foo").unwrap()
        );

        assert_eq!(
            join_url_segments(&base_url, &["/foo", "bar/", "/baz/"]).unwrap(),
            Url::from_str("file:///base/foo/bar/baz").unwrap()
        );

        assert_eq!(
            join_url_segments(&base_url, &["foo/", "", "bar/baz"]).unwrap(),
            Url::from_str("file:///base/foo/bar/baz").unwrap()
        );

        assert_eq!(
            join_url_segments(&base_url, &["foo1/bar1", "foo2/bar2"]).unwrap(),
            Url::from_str("file:///base/foo1/bar1/foo2/bar2").unwrap()
        );
    }

    #[test]
    fn join_failed_due_to_invalid_base() {
        let base_url = Url::from_str("foo:text/plain,bar").unwrap();
        let result = join_url_segments(&base_url, &["foo"]);
        assert!(result.is_err());
    }
}
