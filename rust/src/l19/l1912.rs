use std::collections::{BTreeMap, HashMap};

pub struct MovieRentingSystem {
    all_movies: Vec<Movie>,
    /// HashMap<(shop_id , movie_id) , index>
    movie_index: HashMap<(i32, i32), usize>,
    /// HashSet<movie_id , BTreeMap<Reverse<(price , shop)> , index>,
    avail_movies: HashMap<i32, BTreeMap<(i32, i32), usize>>,
    /// BTreeMap<Reverse<(price , shop , movie)>, index>
    rented_movies: BTreeMap<(i32, i32, i32), usize>,
}

struct Movie {
    shop_id: i32,
    movie_id: i32,
    price: i32,
}

impl Movie {
    fn from_vec(m: Vec<i32>) -> Movie {
        let &[shop_id, movie_id, price] = &m[..] else {
            panic!("invalid input {:?} , length should equal to 3", &m)
        };

        Movie {
            shop_id,
            movie_id,
            price,
        }
    }

    fn index_key(&self) -> (i32, i32) {
        (self.shop_id, self.movie_id)
    }

    fn avail_key(&self) -> (i32, i32) {
        (self.price, self.shop_id)
    }

    fn rented_key(&self) -> (i32, i32, i32) {
        (self.price, self.shop_id, self.movie_id)
    }
}

impl MovieRentingSystem {
    pub fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let all_movies = entries.into_iter().map(Movie::from_vec).collect::<Vec<_>>();

        let mut movie_index: HashMap<(i32, i32), usize> = HashMap::new();
        let mut avail_movies: HashMap<i32, BTreeMap<(i32, i32), usize>> = HashMap::new();
        let rented_movies: BTreeMap<(i32, i32, i32), usize> = BTreeMap::new();

        for (index, movie) in all_movies.iter().enumerate() {
            let curr_avail_movies = avail_movies.entry(movie.movie_id).or_default();
            curr_avail_movies.insert(movie.avail_key(), index);

            movie_index.insert(movie.index_key(), index);
        }

        Self {
            all_movies,
            movie_index,
            avail_movies,
            rented_movies,
        }
    }

    pub fn search(&self, movie: i32) -> Vec<i32> {
        let Some(available_movies) = self.avail_movies.get(&movie) else {
            return vec![];
        };

        available_movies
            .iter()
            .take(5)
            .map(|(_, &index)| self.all_movies[index].shop_id)
            .collect()
    }

    pub fn rent(&mut self, shop: i32, movie: i32) {
        let &movie_index = self.movie_index.get(&(shop, movie)).expect("missing movie");
        let movie = &self.all_movies[movie_index];

        assert!(
            !self.rented_movies.contains_key(&movie.rented_key()),
            "movie rented"
        );

        self.avail_movies
            .get_mut(&movie.movie_id)
            .unwrap()
            .remove(&movie.avail_key())
            .unwrap();

        let rented = self
            .rented_movies
            .insert(movie.rented_key(), movie_index)
            .is_none();

        assert!(rented);
    }

    pub fn drop(&mut self, shop: i32, movie: i32) {
        let &movie_index = self.movie_index.get(&(shop, movie)).expect("missing movie");
        let movie = &self.all_movies[movie_index];

        assert!(
            self.rented_movies.contains_key(&movie.rented_key()),
            "movie not rented"
        );

        self.avail_movies
            .get_mut(&movie.movie_id)
            .unwrap()
            .insert(movie.avail_key(), movie_index);

        self.rented_movies.remove(&movie.rented_key()).unwrap();
    }

    pub fn report(&self) -> Vec<Vec<i32>> {
        self.rented_movies
            .iter()
            .take(5)
            .map(|(_, &index)| {
                let movie = &self.all_movies[index];

                vec![movie.shop_id, movie.movie_id]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn test1() {
        let input =
            common::parse_grid::<i32>("[[0,1,5],[0,2,6],[0,3,7],[1,1,4],[1,2,7],[2,1,5]]").unwrap();
        let mut system = MovieRentingSystem::new(3, input);

        system.search(1);
        system.rent(0, 1);
        system.rent(1, 2);
        system.report();
        system.drop(1, 2);
        system.search(2);
    }
}
