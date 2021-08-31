(ns iqotw.2021.week35)

(defn prefix-words
  "Takes a list of strings and a prefix length. Maps each string in the list
   to the prefix until the prefix length, and returns it as a set."
  [words n]
  (->> words
       (map #(subs % 0 n))
       (set)))

(defn longest-prefix-length
  "Returns the length of the prefix that all of the strings share, or 0 if
  there's no such prefix"
  [words]
  (loop [len 0]
    (let [prefix-set (prefix-words words len)]
      (if (> (count prefix-set) 1)
        (dec len)
        (recur (inc len))))))


(defn longest-prefix
  "Returns the longest prefix shared by all the of the strings in the list,
   or an empty string if no such prefix can be found"
  [words]
  (subs (first words) 0 (longest-prefix-length words)))
