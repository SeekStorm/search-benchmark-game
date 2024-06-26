import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Path;
import java.nio.file.Paths;

import org.apache.lucene.analysis.CharArraySet;
import org.apache.lucene.analysis.standard.StandardAnalyzer;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.index.IndexReader;
import org.apache.lucene.queryparser.classic.MultiFieldQueryParser;
import org.apache.lucene.queryparser.classic.ParseException;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.IndexSearcher;
import org.apache.lucene.search.Query;
import org.apache.lucene.search.TopScoreDocCollector;
import org.apache.lucene.search.similarities.BM25Similarity;
import org.apache.lucene.store.FSDirectory;

public class DoQuery {
    public static void main(String[] args) throws IOException, ParseException {
        final Path indexDir = Paths.get(args[0]);
        try (IndexReader reader = DirectoryReader.open(FSDirectory.open(indexDir));
                BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in))) {
            final IndexSearcher searcher = new IndexSearcher(reader);
            searcher.setQueryCache(null);
            searcher.setSimilarity(new BM25Similarity(0.9f, 0.4f));
            final String[] SEARCH_FIELDS= {"text","title"};
            //final QueryParser queryParser = new QueryParser("text", new StandardAnalyzer(CharArraySet.EMPTY_SET));
            final MultiFieldQueryParser queryParser = new MultiFieldQueryParser(SEARCH_FIELDS, new StandardAnalyzer(CharArraySet.EMPTY_SET));
            String line;
            while ((line = bufferedReader.readLine()) != null) {
                final String[] fields = line.trim().split("\t");
                assert fields.length == 2;
                final String command = fields[0];
                final String query_str = fields[1];
                Query query = queryParser.parse(query_str);
                final int count;
                switch (command) {
                case "COUNT":
                case "UNOPTIMIZED_COUNT":
                    count = searcher.count(query);
                    break;
                case "TOP_10":
                {
                    final TopScoreDocCollector topScoreDocCollector = TopScoreDocCollector.create(10, 10);
                    searcher.search(query, topScoreDocCollector);
                    count = 1;
                }
                break;
                case "TOP_100":
                {
                    final TopScoreDocCollector topScoreDocCollector = TopScoreDocCollector.create(100, 100);
                    searcher.search(query, topScoreDocCollector);
                    count = 1;
                }
                break;
                case "TOP_1000":
                {
                    final TopScoreDocCollector topScoreDocCollector = TopScoreDocCollector.create(1000, 1000);
                    searcher.search(query, topScoreDocCollector);
                    count = 1;
                }
                break;
                case "TOP_10_COUNT":
                {
                    final TopScoreDocCollector topScoreDocCollector = TopScoreDocCollector.create(10, Integer.MAX_VALUE);
                    searcher.search(query, topScoreDocCollector);
                    count = topScoreDocCollector.getTotalHits();
                }
                break;
                case "TOP_100_COUNT":
                {
                    final TopScoreDocCollector topScoreDocCollector = TopScoreDocCollector.create(100, Integer.MAX_VALUE);
                    searcher.search(query, topScoreDocCollector);
                    count = topScoreDocCollector.getTotalHits();
                }
                break;
                case "TOP_1000_COUNT":
                {
                    final TopScoreDocCollector topScoreDocCollector = TopScoreDocCollector.create(1000, Integer.MAX_VALUE);
                    searcher.search(query, topScoreDocCollector);
                    count = topScoreDocCollector.getTotalHits();
                }
                break;
                default:
                    System.out.println("UNSUPPORTED");
                    count = 0;
                    break;
                }
                System.out.println(count);
            }
        }
    }
}
