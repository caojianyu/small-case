package cn.zyx.chat;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

/**
 * @author caojianyu
 * @version 1.0.0
 * @date 2023-01-02 11:42
 * @email jieni_cao@foxmail.com
 */
public class Test {

    public static void main(String[] args) {
        String text1 = "幽灵轰炸机和时间简史的作者";
//        String text2 = "时间简史的作者";
        List<String> wordGroup = new ArrayList<>();
        nGram(text1, wordGroup);
        wordGroup.forEach(System.out::println);
    }

    private static void nGram(String text, List<String> wordGroup) {
        int textLen = text.length();
        for (int i = 2; i < 4; i++) {
            for (int j = 0; j < textLen; j++) {
                int endIndex = i + j;
                if (endIndex > textLen) {
                    continue;
                }
                String word = text.substring(j, endIndex);
                wordGroup.add(word);
            }
        }
    }
}
