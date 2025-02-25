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

WebUI.navigateToUrl(GlobalVariable.URL)

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Mobile Number  Email ID_username'), GlobalVariable.USERNAME)

WebUI.setEncryptedText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Password_password'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/button_Login'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo  Video Consultation with Doctor_95fe55/input_Manage your clinic, answer consult qu_9d0d1b'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo  Video Consultation with Doctor_95fe55/div_Gynecologistobstetrician'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Find Best GynecologistsObstetricians i_0eb5aa/h2_Dr. Gajendra Singh Tomar'))

WebUI.switchToWindowTitle('Dr. Gajendra Singh Tomar - Infertility Specialist - Book Appointment Online, View Fees, Feedbacks | Practo')

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Gajendra Singh Tomar - Infertility_d52b21/button_Stories(333)'))

WebUI.closeBrowser()

