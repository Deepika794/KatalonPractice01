import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.myntra.com/shop/women?utm_source=dms_google&utm_medium=searchbrand_cpc&utm_campaign=dms_google_searchbrand_cpc_Search_Brand_Myntra_Brand_India_BM_TROAS_SOK_New&gad_source=1&gclid=Cj0KCQiAq-u9BhCjARIsANLj-s3bzg1VOyszfQL7TIpVBe9mbci4Eeb4saSDJhjROZyhvHo4Ah9lowsaAtWVEALw_wcB')

WebUI.click(findTestObject('Object Repository/Page_Online Shopping for Women - Shop For W_ab8049/a_Women'))

WebUI.click(findTestObject('Object Repository/Page_Online Shopping for Women - Shop For W_ab8049/a_Dresses'))

WebUI.click(findTestObject('Object Repository/Page_Dresses Online - Low Price Offer on Dr_c05399/img__img-responsive'))

WebUI.switchToWindowTitle('Buy HOUSE OF MIRA Women Floral Printed Shoulder Sleeves A Line Midi Dress - Dresses for Women 31677359 | Myntra')

WebUI.click(findTestObject('Object Repository/Page_Buy HOUSE OF MIRA Women Floral Printed_46d572/a_Jeans'))

WebUI.click(findTestObject('Object Repository/Page_Women Jeans - Buy Jeans for Women Onli_b0afbd/img__img-responsive'))

WebUI.switchToWindowTitle('Buy Trendyol Women Bootcut Flared High Rise Light Fade Stretchable Jeans - Jeans for Women 17813622 | Myntra')

WebUI.click(findTestObject('Object Repository/Page_Buy Trendyol Women Bootcut Flared High_9181ca/p_30'))

WebUI.click(findTestObject('Object Repository/Page_Buy Trendyol Women Bootcut Flared High_9181ca/div_ADD TO BAG'))

WebUI.click(findTestObject('Object Repository/Page_Buy Trendyol Women Bootcut Flared High_9181ca/span_Bag'))

WebUI.closeBrowser()

WebUI.closeBrowser()

