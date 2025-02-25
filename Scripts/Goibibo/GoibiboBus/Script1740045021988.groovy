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

WebUI.navigateToUrl('https://www.goibibo.com/bus/')

WebUI.setText(findTestObject('Object Repository/Goibibo/Page_Bus Booking - Online Bus Ticket Bookin_6cd697/input_FROM_autosuggestBusSRPSrcHomeName'), 
    sourcecity)

WebUI.click(findTestObject('Object Repository/Goibibo/Page_Bus Booking - Online Bus Ticket Bookin_6cd697/span_Pune, Maharashtra'))

WebUI.setText(findTestObject('Object Repository/Goibibo/Page_Bus Booking - Online Bus Ticket Bookin_6cd697/input_TO_autosuggest'), 
    destinationcity)

WebUI.click(findTestObject('Object Repository/Goibibo/Page_Bus Booking - Online Bus Ticket Bookin_6cd697/span_Mumbai, Maharashtra'))

WebUI.click(findTestObject('Object Repository/Goibibo/Page_Bus Booking - Online Bus Ticket Bookin_6cd697/button_Search Bus'))

WebUI.closeBrowser()

