(ns aoc.day15
  (:require [aoc.core :as core])
  (:require [clojure.string :as s]))

;; parse data
(defn parse-line [line]
  (->>
   (re-seq #"-?\d+" line)
   (flatten)
   (mapv #(Integer/parseInt %))
   (partition 2)))

(defn parse [input]
  (->>
   (s/split-lines input)
   (mapv #(vec (parse-line %)))))

;; part one

(defn dist-add [a b]
  (mapv + a b))

(defn dist-sub [a b]
  (mapv - a b))

(defn distance [a b]
  (->>
   (dist-sub a b)
   (mapv abs)
   (reduce +)))

(defn generate-distances [positions]
  (mapv (fn [[sensor beacon]] (vector sensor beacon (distance sensor beacon))) positions))

(defn get-intervals [target data]
  (map (fn [[sx sy] _ d]
         (let [dx (- d (abs (- sy target)))]
           (cond
             (> dx 0) [(- sx dx) (+ sx dx)]
             :else 0)) data)))

(defn compute-count-line [target data])

(defn part1 [input target]
  (->>
   (parse input)
   (generate-distances)
   (compute-count-line target)))

;; part two

(defn part2 [input target]
  (->>
   (parse input)
   (compute-solution-2 target)
   (solution-2)))

;; main
(defn -main []
  (let [input (core/read-file "day15.txt")]
    ; (println "--- Part One ---")
    ; (println "Result:" (part1 input 2000000))
    (println "--- Part Two ---")
    (println "Result:" (part2 input 4000001))))

