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

WebUI.navigateToUrl('https://zolostays.com/')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/zolo/Page_Zolo Coliving - Indias Premier Colivin_6e90a4/div_Gurugram'))

WebUI.click(findTestObject('Object Repository/zolo/Page_PGs in Gurgaon  Boys  Girls PG in Gurgaon/div_Sharing Types_checkbox'))

WebUI.doubleClick(findTestObject('Object Repository/zolo/Page_PGs in Gurgaon  Boys  Girls PG in Gurgaon/img_Sharing Types_Image_styles__Img-sc-j05q_171249'))

WebUI.click(findTestObject('Object Repository/zolo/Page_PGs in Gurgaon  Boys  Girls PG in Gurgaon/div_Sharing Types_checkbox'))

WebUI.click(findTestObject('Object Repository/zolo/Page_PGs in Gurgaon  Boys  Girls PG in Gurgaon/div_Sort By_radio'))

WebUI.closeBrowser()

