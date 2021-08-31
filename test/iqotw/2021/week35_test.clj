(ns iqotw.2021.week35-test
  (:require [clojure.test :as test]
            [iqotw.2021.week35 :as lib]))

(test/deftest prefix-test
  (test/testing "prefix-words maps list of string to their prefixes and 
                 returns them as a set"
    (test/is
     (=
      (lib/prefix-words ["bleed" "blood" "blah" "bleh" "blue"] 2)
      #{"bl"}))
    (test/is
     (=
      (lib/prefix-words ["bleed" "blood" "blah" "bleh" "blue"] 3)
      #{"ble" "blo" "bla" "blu"}))))


(test/deftest longest-prefix-length-test
  (test/testing "longest-prefix-length returns longest common prefix length"
    (test/is
     (=
      (lib/longest-prefix-length ["bleed" "blood" "blah" "bleh" "blue"])
      2))
    (test/is
     (=
      (lib/longest-prefix-length ["bleed" "bleh" "bled" "blend"])
      3))
    (test/is
     (=
      (lib/longest-prefix-length ["bleed" "bleh" "bled" "x"])
      0))))


(test/deftest longest-prefix-test
  (test/testing "longest-prefix returns longest common prefix"
    (test/is
     (=
      (lib/longest-prefix ["bleed" "blood" "blah" "bleh" "blue"])
      "bl"))
    (test/is
     (=
      (lib/longest-prefix ["bleed" "bleh" "bled" "blend"])
      "ble"))
    (test/is
     (=
      (lib/longest-prefix ["x" "bleh" "bled" "blend"])
      ""))))

