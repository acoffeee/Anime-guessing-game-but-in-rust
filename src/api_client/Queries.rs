
pub const USERLISTGUESSINGQUERY: &str = "
query ($userName: String)
{
  MediaListCollection (userName: $userName, type:ANIME, status:COMPLETED) {
    lists {
      entries {
        score(format: POINT_100)
        media {
        id
        season
        seasonYear
        format
        genres
        tags {
          name
          rank
        }
        averageScore
        source
        title {
          romaji
          english
        }
        }
      }
    }
  }
}
";
pub const ANIMEINFORMATIONQUERY: &str = "
query($id: Int)
{
  Media(id: $id)
  {
    id
    season
    seasonYear
    format
    genres
    tags {
      name
      rank
    }
    averageScore
    source
    title {
      romaji
      english
    }
  }
}
";

pub const USERIDQUERY: &str = "
query ($userName: String)
{
  User(name: $userName) {
    id
  }
}
";

pub const FOLLOWINGQUERY: &str = "
query ($page: Int, $userId: Int!)
{
  Page(page: $page, perPage: 50) {
    following(userId: $userId) {
      id
      name
    }
  }
}
";

pub const USERLISTINFOQUERY: &str = "
query ($userName: String)  
{
  MediaListCollection (userName: $userName, type:ANIME) {
    lists {
      entries {
        score
        media {
        id
        }
      }
    }
  }
}
";

pub const FAVSQUERY: &str = "
query ($userName: String, $page: Int) {
  User(name: $userName) {
    favourites {
        anime (page: $page, perPage: 25) {
            nodes {
                id
            }
            pageInfo {
                hasNextPage
            }
        }
    }
  }
}
";